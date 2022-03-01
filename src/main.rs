extern crate core;

mod input;
mod network;
mod yaml;
mod ui;
mod input_management;
mod utils;

use std::env;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use ed25519_dalek::Keypair;
use encoins_api::base_types::UserId;
use crate::input_management::{deal_with_input, parse_input};
use crate::ui::{show_terminal};
use crate::utils::{load_key_pair};


fn main()
{
    // A wallet path can be given as a first argument
    let args: Vec<String> = env::args().collect();

    // Vector containing additional strings to be outputted on screen under the logo
    let mut additional_strings = vec![];

    // Current TcpStream used to communicate with one of the servers
    // let mut stream : Option<TcpStream> = None;

    // Sender/Receiver to deal with responses from servers
    let (main_sender, main_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

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
                            additional_strings.push(format!("Successfully loaded wallet for user {}", UserId::from_bytes(kp.public.to_bytes())));
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
        show_terminal(&additional_strings);
        match parse_input(&user_keypair)
        {
            Ok(inp) =>
                {
                    deal_with_input(inp, &mut additional_strings, &main_sender, &mut user_keypair);
                }
            Err(err) =>
                {
                    additional_strings.push(err);
                }
        }
        update_responses(&mut additional_strings, &main_receiver);

    }

}

fn update_responses(additional_strings: &mut Vec<String>, main_receiver : &Receiver<String>)
{
    loop
    {
        match main_receiver.try_recv()
        {
            Ok(str) =>
                {
                    additional_strings.push(str);
                }
            Err(_) =>
                {
                    break;
                }
        }
    }
}