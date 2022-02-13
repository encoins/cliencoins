mod input;
mod base_types;
mod network;
mod crypto;
mod client;
mod instructions;
mod message;
mod response;
mod yaml;

use std::fs::read;
use std::net::TcpStream;
use std::io::{Write, Read, stdin, stdout};
use crate::base_types::UserId;
use crate::client::Client;
use crate::input::Input;
use crate::network::exchange_with_server;
use std::env;






fn main() {
    let id: UserId = env::var("NUM_NODE")
        .expect("No environment variable NUM_NODE found")
        .parse::<u32>().unwrap()
        as UserId;

    let client = Client::new(id);
    println!("Bienvenue client {:?}! Merci d'avoir choisi ENcoinS",client.id);
    network::connect_to_serv(&client)
}