use std::net::{TcpStream};
use std::io::{Write};
use crate::yaml::*;
use crate::instructions::Instruction;
use crate::UserId;

pub fn connect_to_network(stream: &mut Option<TcpStream>)
{
    match stream
    {
        Some(_) => { return; }
        None    =>
            {
                let hash_net_config = yaml_to_hash("net_config.yml");
                let nb_nodes = read_network_parameters(&hash_net_config);

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
    }
}

pub fn ask_balance(user: UserId, stream : &mut  Option<TcpStream> ) -> bool
{
    // Make sure the client is connected to the network
    connect_to_network(stream);

    match stream
    {
        Some(tcpstream) =>
            {
                // Create the ask balance message and send it
                let msg = &(bincode::serialize(&Instruction::Balance {user}).unwrap()[..]);
                match tcpstream.write(msg)
                {
                    Ok(_) => {  true }
                    Err(_) => {  false }
                }
            }
        None => { false }
    }
}

pub fn transfer_request(stream : &mut  Option<TcpStream>, transfer : Instruction) -> bool
{
    // Make sure the client is connected to the network
    connect_to_network(stream);
    match stream
    {
        Some(tcpstream) =>
            {
                // Create the transfer request and send it
                let msg = &(bincode::serialize(&transfer).unwrap()[..]);
                tcpstream.write(msg);
                true
            }
        None => { false }
    }
}
