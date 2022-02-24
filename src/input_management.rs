use std::io;
use std::io::Write;
use std::net::TcpStream;
use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::Keypair;
use crate::{Input, ui};
use crate::base_types::Transfer;
use crate::network::{ask_balance, transfer_request};
use crate::ui::show_terminal;
use crate::utils::loadKeyPair;

/// Parses an input from terminal and returns an Input
pub fn parse_input(strings_to_show : &mut Vec<String>, user_keypair : Option<Keypair>) -> Input
{

    // Loops until no correct inputs has been entered
    loop
    {
        let mut input_line = String::new();
        let words: Vec<&str>;

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        // Deletion of the last character : '\n'
        let len = input_line.len();

        // Parsing of the input line as an op_type and an array args of arguments, managing the syntax errors
        words = input_line[..len-1].split(' ').collect();

        let input = Input::from(&words, );


        match input
        {
            Ok(input) =>
                {
                    return input
                }
            Err(string_error) =>
                {
                    // Print error message and ask for another input
                    println!("{}", string_error);
                    print!("> ");
                    io::stdout().flush().unwrap()
                }
        }
    }

}

pub fn deal_with_input(input : Input, strings_to_show: &mut Vec<String>, stream: &mut Option<TcpStream>, user_keypairs: &mut Option<Keypair>)
{
    match input
    {
        Input::Transfer { sender, recipient, amount } =>
            {
                let mut transfer = Transfer
                {
                    sender,
                    recipient,
                    amount
                };

                match user_keypairs
                {
                    None =>
                        {
                            strings_to_show.push(String::from("Please load a wallet to make a transfer request!"));
                        }
                    Some(keypairs) =>
                        {
                            // Normally should if one of the keypair is the good one and sign with it. At the moment we suppose we can just sign with whatever key we have
                            let sgn_transfer = transfer.sign(keypairs);
                            transfer_request(stream, sgn_transfer);
                        }
                }

            }
        Input::Help =>
            {
                ui::show_help();
            }
        Input::Clear =>
            {
                strings_to_show.clear();
            }
        Input::Quit =>
            {
                println!("Goodbye!");
                std::process::exit(0);
            }

        Input::Balance { user } =>
            {
                match ask_balance(user,stream)
                {
                    true => {}
                    false =>
                        {
                           strings_to_show.push(String::from("Could not connect to a server!"));
                        }
                }
            }
        Input::LoadWallet { path} =>
            {
                match loadKeyPair(&path)
                {
                    Ok(keypairs) =>
                        {
                            user_keypairs.replace(keypairs);
                        }
                    Err(err) => { strings_to_show.push(err) }
                }
            }
    }
}
