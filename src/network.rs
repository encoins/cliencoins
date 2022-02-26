use std::net::{TcpStream};
use std::io::{Write,Read};
use crate::yaml::*;
use crate::instructions::Instruction;
use crate::response::Response;
use crate::UserId;
use bincode::deserialize;


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

pub fn send_instruction(stream : &mut Option<TcpStream>, instruction : Instruction) -> Response
{
    let mut buf = [0; 100];
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