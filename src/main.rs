mod input;
mod base_types;
mod network;
mod crypto;
mod client;
mod instructions;
mod message;
mod response;
mod yaml;
mod ui;
mod input_management;
mod transfer;
mod utils;
mod pub_key_converter;

use std::fs::read;
use std::net::TcpStream;
use std::io::{Write, Read, stdin, stdout};
use crate::base_types::UserId;
use crate::input::Input;
use std::env;
use ed25519_dalek::Keypair;
use serde::Serialize;
use crate::input_management::{deal_with_input, parse_input};
use crate::ui::{print_logo, show_terminal};


fn main()
{
    // Vector containing additional strings to be outputted on screen under the logo
    let mut additional_strings = vec![];
    let mut stream : Option<TcpStream> = None;
    let mut user_keypair : Option<Keypair> = None;

    loop
    {
        print_logo();
        show_terminal(&mut additional_strings);
        let input : Input = parse_input(&mut additional_strings);
        deal_with_input(input, &mut additional_strings, &mut stream, &mut user_keypair)
    }

}