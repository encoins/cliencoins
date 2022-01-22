mod input;
mod base_types;
mod network;

use std::fs::read;
use std::net::TcpStream;
use std::io::{Write, Read, stdin, stdout};
use crate::input::Input;
use crate::network::exchange_with_server;





fn main() {
    network::connect_to_serv()
}