//! Definition of the input enum defining regular inputs

use crate::base_types::{Currency, UserId};
use std::io;
use crate::utils::{string_to_user_id};
use std::io::Write;
use ed25519_dalek::{Keypair, PublicKey};
use serde::de::Unexpected::Str;

/// An input can either be a transfer or balance request or an interaction with the GUI
pub enum Input
{

    Transfer{ sender : UserId, recipient: UserId, amount : Currency },
    /// Input to get transactions history of an account according to a given account
    Help,
    /// Input to clear terminal from previous inputs
    Clear,
    /// Input to quit program
    Quit,
    Balance { user : UserId },
    LoadWallet { path : String},
    GenWallet { path : String}

}

impl Input
{
    const WRONG_AMOUNT_OF_ARGS: &'static str = "Wrong amount of arguments! (Type \"help\" to see how to use command)";

    pub fn from(value: &Vec<&str>, sender_keypair: &Option<Keypair>) -> Result<Input,String>
    {
        let mut args = vec![];
        return if value.len() == 0
        {
            Err(String::from("No command entered! Type \"help\" to get a list of possible commands"))
        } else {
            for k in 1..value.len()
            {
                let word = String::from(value[k].trim());
                args.push(word);
            }

            // Transforms first argument to lowercase
            let value_lc = &value[0].to_lowercase()[..];

            match value_lc
            {


                "transfer" =>
                    {
                        if args.len() != 2
                        {
                            return Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        }
                        else
                        {

                            match sender_keypair
                            {
                                None =>
                                    {
                                        return Err( String::from("Please load a wallet before trying to make a transfer!") )
                                    }
                                Some(key) =>
                                    {
                                        let recipient_key : PublicKey = PublicKey::from_bytes(string_to_user_id(&args[0]).as_ref()).unwrap();
                                        let amount : u32 = args[1].parse().unwrap();
                                        Ok( Input::Transfer {sender: key.public.to_bytes(), recipient: recipient_key.to_bytes(), amount: amount } )
                                    }
                            }


                        }
                    }

                "help" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Help)
                        }
                    }
                "clear" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Clear)
                        }
                    }
                "quit" =>
                    {
                        if args.len() != 0
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Quit)
                        }
                    }

                "balance" =>
                    {
                        match args.len()
                        {
                            0 =>
                                {
                                    match sender_keypair
                                    {
                                        None =>
                                            {
                                                return Err(String::from("Please load a wallet or explicit a public key from which you want to know the balance !"))
                                            }
                                        Some(key) =>
                                            {
                                                return Ok(Input::Balance {user: key.public.to_bytes()})
                                            }
                                    }
                                }
                            1 =>
                                {
                                    return Ok(Input::Balance { user: string_to_user_id(&args[0]) })
                                }

                            _ =>
                                {
                                    return Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                                }
                        }

                    }

                "ldwallet" =>
                    {
                        if args.len()!= 1
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        }
                        else
                        {
                            Ok(Input::LoadWallet { path : args[0].clone() })
                        }
                    }

                "genwallet" =>
                    {
                        if args.len()!= 1
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        }
                        else
                        {
                            Ok(Input::GenWallet { path : args[0].clone() })
                        }
                    }
                _ =>
                    {
                        Err(String::from("The typed command could not be recognised! (Type \"help\" to get a list of possible commands)"))
                    }
            }
        }
    }

}

fn parse_args_as<T: std::str::FromStr>(args: Vec<String>) -> Result<Vec<T>, String>
{
    let mut ars: Vec<T> = vec![];
    for tmp_arg in args
    {
        let arg: T = match tmp_arg.parse()
        {
            Ok(obj) => obj,
            Err(_) =>
                {
                    return Err(String::from("Arguments have the wrong type! (Type \"help\" to see how to use command)"))
                }
        };
        ars.push(arg);
    }

    return Ok(ars)
}