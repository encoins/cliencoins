use std::io;
use std::net::TcpStream;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use crate::{Input, ui};
use crate::base_types::Transfer;
use crate::network::{ask_balance, transfer_request};
use crate::utils::{export_key_pair, load_key_pair, user_id_to_string};

/// Parses an input from terminal and returns an Input
pub fn parse_input(user_keypair : &Option<Keypair>) -> Result<Input, String>
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

    Input::from(&words, user_keypair)
}

pub fn deal_with_input(input : Input, strings_to_show: &mut Vec<String>, stream: &mut Option<TcpStream>, user_keypairs: &mut Option<Keypair>)
{
    match input
    {
        Input::Transfer { sender, recipient, amount } =>
            {
                let transfer = Transfer
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
                            let sgn_transfer = transfer.sign(keypairs);
                            match transfer_request(stream, sgn_transfer)
                            {
                                true =>
                                    {
                                        strings_to_show.push(format!("Everything is good. Making the transfer request of {} encoins to user {} ", amount, user_id_to_string(&recipient)));
                                    }
                                false =>
                                    {
                                        strings_to_show.push(String::from("Could not connect to a server ! "));
                                    }
                            }
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
                    true =>
                        {
                            strings_to_show.push(String::from("TEST!"));
                        }
                    false =>
                        {
                           strings_to_show.push(String::from("Could not connect to a server!"));
                        }
                }
            }
        Input::LoadWallet { path} =>
            {
                match load_key_pair(&path)
                {
                    Ok(keypairs) =>
                        {
                            strings_to_show.push(format!("Successfully loaded wallet for user {}", user_id_to_string(&keypairs.public.to_bytes())));
                            user_keypairs.replace(keypairs);
                        }
                    Err(err) => { strings_to_show.push(err) }
                }
            }

        Input::GenWallet { path } =>
            {
                let mut csprng = OsRng{};
                let keypair: Keypair = Keypair::generate(&mut csprng);
                export_key_pair(&path, &keypair );
                strings_to_show.push(format!("Successfully generate the wallet {}. The wallet has also been loaded as current wallet.", path));
                user_keypairs.replace(keypair);
            }
    }
}
