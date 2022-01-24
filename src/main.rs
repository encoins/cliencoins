mod input;
mod base_types;
mod network;
mod crypto;
mod client;
mod instructions;
mod message;

use std::fs::read;
use std::net::TcpStream;
use std::io::{Write, Read, stdin, stdout};
use crate::client::Client;
use crate::input::Input;
use crate::network::exchange_with_server;





fn main() {
    let client = Client::new();
    network::connect_to_serv(&client)
}