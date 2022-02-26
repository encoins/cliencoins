mod input;
mod base_types;
mod network;
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

use std::net::TcpStream;
use crate::base_types::UserId;
use crate::input::Input;
use std::env;
use ed25519_dalek::Keypair;
use crate::input_management::{deal_with_input, parse_input};
use crate::ui::{show_terminal};
use crate::utils::{load_key_pair, user_id_to_string};


fn main()
{
    // A wallet path can be given as a first argument
    let args: Vec<String> = env::args().collect();

    // Vector containing additional strings to be outputted on screen under the logo
    let mut additional_strings = vec![];
    // Current TcpStream used to communicate with one of the servers
    let mut stream : Option<TcpStream> = None;

    // Current Keypair (User). Can be None if no user is connected
    let mut user_keypair : Option<Keypair> = match args.get(1)
    {
        None => { None }
        Some(path ) =>
            {
                match load_key_pair(path)
                {
                    Ok(kp) =>
                        {
                            additional_strings.push(format!("Successfully loaded wallet for user {}", user_id_to_string(&kp.public.to_bytes())));
                            Some(kp)
                        }
                    Err(err) =>
                        {
                            additional_strings.push(err);
                            None
                        }
                }
            }
    };

    // Main loop where we juste wait for inputs and deal with them
    loop
    {
        match parse_input(&user_keypair)
        {
            Ok(inp) =>
                {
                    deal_with_input(inp, &mut additional_strings, &mut stream, &mut user_keypair);
                }
            Err(err) =>
                {
                    additional_strings.push(err);
                }
        }

    }

}