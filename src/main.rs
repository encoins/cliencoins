mod input;
mod base_types;
mod network;
mod crypto;
mod client;
mod instructions;
mod message;
mod response;

use std::fs::read;
use std::net::TcpStream;
use std::io::{Write, Read, stdin, stdout};
use crate::base_types::UserId;
use crate::client::Client;
use crate::input::Input;
use crate::network::exchange_with_server;
use std::env;






fn main() {
    let args: Vec<String> = env::args().collect();
    let id = args[1].parse::<u32>().unwrap() as UserId;

    let client = Client::new(id);
    println!("My id is {:?}",client.id);
    network::connect_to_serv(&client)
}