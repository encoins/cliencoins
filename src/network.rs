//! Deals with communications with the encoins network

use core::time;
use std::collections::HashMap;
use std::net::{TcpStream};
use std::io::{Write,Read};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::yaml::*;
use crate::instructions::Instruction;
use crate::response::Response;
use bincode::{deserialize};
use serde::de::DeserializeOwned;
use rand::random;
use crate::base_types::Currency;
use crate::UserId;

/// Connects the client to a random node of the network using network parameters given in the `net_config.yml` file
pub fn connect_to_network(stream: &mut Option<TcpStream>)
{
    // Checks if the given stream exists and is still alive
    match stream
    {
        Some(tcp) =>
            {
                // Checks if the stream is still alive
                match tcp.write("".as_ref())
                {
                    Ok(_) =>
                        {
                            // Returns here only if the stream is alive
                            return;
                        }
                    Err(_) => {}
                }
            }
        None    => {}
    }

    let hash_net_config = yaml_to_hash("net_config.yml");
    let nb_nodes = read_network_parameters(&hash_net_config);

    let rd_start: u32 = random::<u32>()%nb_nodes +1;
    // Tries to connect to each server until it achieves to do so.
    for i in 1..nb_nodes+1
    {
        let id_node = (i +rd_start)%nb_nodes;
        let address = read_server_address(&hash_net_config, id_node);
        //println!("Trying to connect to node {}...", id_node);

        let temp_stream = TcpStream::connect(address);
        match temp_stream
        {
            Ok(str) =>
                {

                    stream.replace(str);
                    return;
                }
            Err(_) =>
                {
                    //println!("Connection to node {} failed", id_node);
                }
        }

    }
}

/// Sends an instruction given by the client to a server
pub fn send_instruction(stream : &mut Option<TcpStream>, instruction : Instruction) -> Response
{
    let mut buf = [0; 100];
    // Makes sure the client is connected to a server
    connect_to_network(stream);
    match stream
    {
        Some(tcpstream) =>
            {

                let msg = &(bincode::serialize(&instruction).unwrap()[..]);
                tcpstream.write(msg);

                match tcpstream.read(&mut buf) {
                    Ok(received) => {
                        if received < 1 {
                           return  Response::RcvErr;
                        }
                    }
                    Err(_) => {
                        return Response::RcvErr;
                    }

                }
                let response : Response = deserialize(&buf[..]).unwrap();
                response

            }
        None => { Response::SendErr }
    }
}



pub fn get_balance(user : &UserId, main_sender : &Sender<String>)
{
    let hash_net_config = yaml_to_hash("net_config.yml");
    let nb_nodes = read_network_parameters(&hash_net_config);
    let mut nb_answers = 0;
    let mut answers : HashMap<Currency, u32> = HashMap::new();
    let us = user.clone();
    let local_ms = main_sender.clone();
    // Spawn a thread to deal with balance
    thread::spawn(
        move ||
            {
                // Create the sender and receiver to communicate with newly spawned threads
                let (sender, receiver) : (Sender<Result<Response,String>>, Receiver<Result<Response,String>>) = mpsc::channel();



                // Spawn response thread for each server
                for i in 1..(nb_nodes+1)
                {
                    let sender_cop = sender.clone();
                    // Function that deals with the server response
                    let response_dealer  = move |result : Result<Response, String>|
                        {

                            sender_cop.send(result).unwrap();
                        };
                    let instruction = Instruction::Balance { user : us.clone() };
                    let address = read_server_address(&hash_net_config, i);
                    spawn_response_thread(instruction, Some(address), response_dealer, None );
                }

                // Now deal with the response
                while nb_answers != nb_nodes
                {
                    match receiver.recv()
                    {
                        Ok(res) =>
                            {
                                nb_answers+=1;
                                match res
                                {
                                    Ok(res) =>
                                        {
                                            match res
                                            {
                                                Response::Balance(cur) =>
                                                    {
                                                        match answers.get(&cur)
                                                        {
                                                            None =>
                                                                {
                                                                    answers.insert(cur, 1);
                                                                }
                                                            Some(j) =>
                                                                {
                                                                    answers.insert(cur, j+1);
                                                                }
                                                        }
                                                    }
                                                _ => {}
                                            }
                                        }
                                    Err(_) => {}
                                }
                            }
                        Err(err) =>
                            {
                                panic!("Fatal error while trying to make the balance request : {}", err.to_string());
                            }
                    }
                }

                // Transform response into a string to return
                let mut answ_it = answers.into_iter();
                let mut rtn_str = String::new();
                match answ_it.len()
                {
                    0 =>
                        {
                            panic!("Fatal error while trying to make the balance request : no value returned");
                        }
                    1 =>
                        {
                            rtn_str = format!("All nodes in the network agree that the user has {} encoins", answ_it.next().unwrap().0);
                        }
                    _ =>
                        {
                            rtn_str.push_str("Nodes don't all agree on the balance of the user : ");
                            for answ in answ_it
                            {
                                rtn_str = format!("{}\n- {} according to {} servers", rtn_str, answ.0, answ.1);
                            }
                        }
                }

                // Send String to main
                local_ms.send(rtn_str);
            }


    );
}

pub fn make_transfer(instruction : Instruction, main_sender: &Sender<String>)
{
    let ms_copy = main_sender.clone();
    let response_dealer  = move |result : Result<Response, String>|
    {
        match result
        {
            Ok(response) =>
                {
                    ms_copy.send(response.to_string());
                }
            Err(str) =>
                {
                     ms_copy.send(str).unwrap();
                }
        }
    };

    spawn_response_thread(instruction, None, response_dealer, None);
    return;
}

fn spawn_response_thread<
    MessageT:serde::Serialize + Send + 'static,
    ResponseT: DeserializeOwned + 'static,
    ResponseHandlingT: Fn(Result<ResponseT, String>) + Send + 'static
    >
(
    message : MessageT,
    server_address : Option<(String,u16)>,
    response_handler: ResponseHandlingT,
    max_wait_time : Option<u64>
)
{
    thread::spawn(
        move ||
            {
                // First connect to a specific or a random server
                let mut stream;
                match server_address
                {
                    None =>
                        {
                            let mut part_stream : Option<TcpStream>= None;
                            loop
                            {
                                match part_stream
                                {
                                    None =>
                                        {
                                            connect_to_network(&mut part_stream);
                                        }
                                    Some(tcp) =>
                                        {
                                            stream = tcp;
                                            break;
                                        }
                                }
                            }
                        }
                    Some(address) =>
                        {
                            loop
                            {
                                match TcpStream::connect(address.clone())
                                {
                                    Ok(tcpstr) =>
                                        {
                                            stream = tcpstr;
                                            break;
                                        }
                                    Err(_) => {}
                                }

                            }
                        }
                };

                // Then send the desired request
                let msg = &(bincode::serialize(&message).unwrap()[..]);
                stream.write(msg);

                let wait_time  = match max_wait_time // in milliseconds
                {
                    None =>
                        {
                            time::Duration::from_millis(10000)
                        }
                    Some(time) =>
                        {
                            time::Duration::from_millis(time)
                        }
                };

                // Loop to try receiving the message for a given wait time
                let mut nb_wait = 0;
                let mut buf;
                let mut received = false;
                loop
                {
                    buf = [0;100];
                    match stream.read(&mut buf)
                    {
                        Ok(rcv_size) =>
                            {
                                if rcv_size < 1 {}
                                else
                                {
                                    // Stop looping
                                    received = true;
                                    break;
                                }
                            }
                        Err(_) => { } // Continue looping
                    }
                    nb_wait+=1;
                    thread::sleep(time::Duration::from_millis(50));

                    if nb_wait*50 > wait_time.as_millis()
                    {
                        break;
                    }

                }

                // Send a response if there is one
                if !received
                {
                   response_handler(Err(format!("No response from server")));
                }
                else
                {
                    let response : bincode::Result<ResponseT> = deserialize(&buf[..]);
                    match response
                    {
                        Ok(rcv) =>
                            {
                                response_handler(Ok(rcv));
                            }
                        Err(_) =>
                            {
                                response_handler(Err(format!("Error when trying to read response from server")));
                            }
                    }
                }
            }
    );
}
