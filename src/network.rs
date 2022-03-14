//! Deals with communications with the encoins network

use core::time;
use std::collections::HashMap;
use std::net::{TcpStream};
use std::io::{Write,Read};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::yaml::*;
use encoins_api::instruction::Instruction;
use encoins_api::base_types::{Currency, UserId};
use encoins_api::response::Response;
use bincode::{deserialize};
use serde::de::DeserializeOwned;
use rand::random;

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

    let rd_start: u32 = random::<u32>()%nb_nodes;
    // Tries to connect to each server until it achieves to do so.
    for i in 0..nb_nodes
    {
        let id_node = (i +rd_start)%nb_nodes +1 ;
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

/// Gets the balance of a given user by broadcasting a request to every servers
pub fn get_balance(user : &UserId, main_sender : &Sender<String>)
{
    let hash_net_config = yaml_to_hash("net_config.yml");
    let nb_nodes = read_network_parameters(&hash_net_config);
    let mut nb_answers = 0;
    let mut nb_error_answ = 0;
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
                for i in 0..nb_nodes
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
                    // The threads above will send their result in the given receiver

                    match receiver.recv().expect("Fatal error: this should never happen!")
                    {
                        // This thread should always be able to receive.

                        Ok(response) =>
                            {
                                match response
                                {
                                    Response::Balance(cur) =>
                                        {
                                            // This code block increments by one the number of servers agreeing for amount "cur" in the hashmap answers
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

                        Err(_) =>
                            {
                                nb_error_answ +=1;
                            }

                    }

                    nb_answers+=1;
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
                            if nb_error_answ >0
                            {
                                rtn_str = format!("All nodes that answered agree that the user has {} encoins ({} servers did not answer in time)", answ_it.next().unwrap().0, nb_error_answ);
                            }
                            else
                            {
                                rtn_str = format!("All nodes in the network agree that the user has {} encoins", answ_it.next().unwrap().0);
                            }

                        }
                    _ =>
                        {
                            rtn_str.push_str("Nodes don't all agree on the balance of the user : ");
                            if nb_error_answ > 0
                            {
                                rtn_str = format!("{}({} servers did not answer in time)", rtn_str, nb_error_answ)
                            }
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

/// Makes a transfer request to a randomly chosen server
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

/// Spawns a thread that deals with the communication of a given message
///
/// # Generic arguments
///
/// * `MessageT` - Type of the message to send to a server
/// * `ResponseT` - Type of the expected server's response
/// * `ResponseHandlingT` - Type of the enclosure used to handle the server's response
///
/// # Arguments
///
/// * `message` - Message to be sent to a server
/// * `server_address` - Address of the server to send the message to. A random server will be chosen if it has `None` value
/// * `response_handler` - Enclosure to be used to deal with the server's response
/// * `thread_lifetime` - Maximum time (in milliseconds) that the thread will wait for the server's response before ending. Lifetime is set to 10 seconds if it has `None` value
///
/// # Examples
///
/// See [`make_transfer`]
fn spawn_response_thread<
    MessageT:serde::Serialize + Send + 'static,
    ResponseT: DeserializeOwned + 'static,
    ResponseHandlingT: Fn(Result<ResponseT, String>) + Send + 'static
    >
(
    message : MessageT,
    server_address : Option<(String,u16)>,
    response_handler: ResponseHandlingT,
    thread_lifetime: Option<u64>
)
{
    thread::spawn(
        move ||
            {
                // First determine thread's lifetime
                let mut remaining_lifetime = match thread_lifetime // in milliseconds
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


                // Then try to connect to a specific or a random server.
                let mut opt_stream = None;
                let mut time_looped = Duration::new(0, 0);
                match server_address
                {
                    None =>
                        {
                            let mut part_stream : Option<TcpStream>= None;
                            let loop_start = std::time::Instant::now();
                            // Loop while the client does not manage to connect to a server
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
                                            opt_stream = Some(tcp);
                                            remaining_lifetime -= time_looped;
                                            break;
                                        }
                                }

                                time_looped = std::time::Instant::now() - loop_start;

                                if time_looped > remaining_lifetime
                                {
                                    response_handler(Err(String::from("Could not connect to a server! (Are you connected to the internet?)")));
                                    break;
                                }
                            }
                        }
                    Some(address) =>
                        {
                            let loop_start = std::time::Instant::now();
                            loop
                            {
                                match TcpStream::connect(address.clone())
                                {
                                    Ok(tcpstr) =>
                                        {
                                            opt_stream = Some(tcpstr);
                                            remaining_lifetime -= time_looped;
                                            break;
                                        }
                                    Err(_) => {}
                                }

                                time_looped = std::time::Instant::now() - loop_start;

                                if time_looped > remaining_lifetime
                                {
                                    response_handler(Err(format!("Could not connect to the server {}! (The server is byzantine or you are not connected to the internet)", address.0)));
                                    break;
                                }

                            }
                        }
                };


                match opt_stream
                {
                    None => {}
                    Some(mut stream) =>
                        {
                            // Then send the desired request
                            let msg = &(bincode::serialize(&message).unwrap()[..]);
                            stream.write(msg);


                            // Loop to try receiving the message for a given wait time
                            let loop_start = std::time::Instant::now();
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
                                thread::sleep(time::Duration::from_millis(50));

                                time_looped = std::time::Instant::now() - loop_start;

                                if time_looped > remaining_lifetime
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
                }
            }
    );
}
