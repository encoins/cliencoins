//! Deals with communications with the encoins network

use std::net::{TcpStream};
use std::io::{Write,Read};
use crate::yaml::*;
use crate::instructions::Instruction;
use crate::response::Response;
use bincode::deserialize;

/// Connects the client to the network using network parameters given in the `net_config.yml` file
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

    // Tries to connect to each server until it achieves to do so.
    for i in 1..nb_nodes+1
    {
        let id_node = i;
        let address = read_server_address(&hash_net_config, id_node);
        println!("Trying to connect to node {}...", id_node);

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
                    println!("Connection to node {} failed", id_node);
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