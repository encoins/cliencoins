extern crate core;
mod input;
mod network;
mod yaml;
mod ui;
mod input_management;
mod utils;

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::{Duration, Instant};
use ed25519_dalek::Keypair;
use encoins_api::base_types::UserId;
use crate::input::Input;
use crate::input_management::{deal_with_input, parse_input};
use crate::ui::{show_terminal};
use crate::utils::{load_key_pair};


fn main()
{
    let start_time = Instant::now();
    let mut waiting_responses:u32 = 0;

    // A wallet path can be given as a second argument
    let args: Vec<String> = env::args().collect();

    // Used to time program execution

    // Vector containing additional strings to be outputted on screen under the logo
    let mut additional_strings = vec![];

    // Current TcpStream used to communicate with one of the servers
    // let mut stream : Option<TcpStream> = None;

    // Sender/Receiver to deal with responses from servers
    let (main_sender, main_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

    // Current Keypair (User). Can be None if no user is connected
    let mut user_keypair : Option<Keypair> = match args.get(2)
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

    let input_terminal : bool = match args.get(1)
    {
        None =>
            {
                true
            }
        Some(b) =>
            {
                b.parse().unwrap()
            }
    };

    // Main loop where we juste wait for inputs and deal with them
    loop
    {
        if input_terminal
        {
            show_terminal(&additional_strings);
        }
        match parse_input(&user_keypair)
        {
            Ok(inp) =>
                {
                    match inp
                    {
                        Input::Quit =>
                            {
                                quit(start_time, &mut waiting_responses, &main_receiver);
                            }
                        Input::Transfer {.. } | Input::Balance{..} =>
                        {
                            waiting_responses +=1;
                        }
                        _ => {}
                    }

                    deal_with_input(inp, &mut additional_strings, &main_sender, &mut user_keypair);
                }
            Err(err) =>
                {
                    if input_terminal
                    {
                        additional_strings.push(err);
                    }
                    else
                    {
                        println!("{}",err);
                    }
                }
        }
        update_responses(&mut additional_strings, &main_receiver, &mut waiting_responses);

    }

}

fn update_responses(additional_strings: &mut Vec<String>, main_receiver : &Receiver<String>, waiting_responses: &mut u32)
{
    loop
    {
        match main_receiver.try_recv()
        {
            Ok(str) =>
                {

                    *waiting_responses-= 1;
                    println!("Got response : {} ; Subastracting one, remaining : {}", str, waiting_responses);
                    additional_strings.push(str);

                }
            Err(_) =>
                {
                    break;
                }
        }
    }
}

fn quit(start_instant : Instant, waiting_responses: &mut u32, main_receiver : &Receiver<String>)
{
    while *waiting_responses != 0
    {
        println!("Waiting for {} responses", waiting_responses);
        match main_receiver.recv()
        {
            Ok(str) =>
                {
                    *waiting_responses-=1;
                    println!("Got response : {} ; Subastracting one, remaining : {}", str, waiting_responses);
                }
            Err(err) =>
                {
                    println!("Error : {}", err.to_string());
                }

        }
        sleep(Duration::from_millis(10));
    }
    let elapsed_time = Instant::now() - start_instant;
    println!("\n{}", elapsed_time.as_millis());
    std::process::exit(0);
}