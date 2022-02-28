//! Definition of the input enum defining regular inputs

use crate::base_types::{Currency, UserId};
use crate::utils::{string_to_user_id};
use ed25519_dalek::{Keypair, PublicKey};

/// An input can either be a transfer or balance request or an interaction with the GUI
pub enum Input
{

    /// Ask for a money transfer
    Transfer{ sender : UserId, recipient: UserId, amount : Currency },
    /// Get the balance of a given user
    Balance { user : UserId },
    /// Load an encoins wallet
    LoadWallet { path : String},
    /// Generate an encoins wallet
    GenWallet { path : String},
    /// Get help screen
    Help,
    /// Clear terminal from previous inputs
    Clear,
    /// Quit program
    Quit,
    ///
    None


}

impl Input
{

    const WRONG_AMOUNT_OF_ARGS: &'static str = "Wrong amount of arguments! (Type \"help\" to see how to use command)";

    /// Builds an [`Input`] from an array of strings and an optional [`Keypair`]
    /// # Errors
    /// This function will return an error if the given string vector is empty or if it did not manage
    /// to parse the content of the given vector. It will also return an error if the user is trying
    /// to make a transfer and no keypair was given.
    pub fn from(value: &Vec<&str>, sender_keypair: &Option<Keypair>) -> Result<Input,String>
    {
        let mut args = vec![];
        return
            if value.len() == 0
            {
                Err(String::from("No command entered! Type \"help\" to get a list of possible commands"))
            }
            else
            {
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
                            } else {
                                match sender_keypair
                                {
                                    None =>
                                        {
                                            return Err(String::from("Please load a wallet before trying to make a transfer!"))
                                        }
                                    Some(key) =>
                                        {
                                            let parsed_uid = match string_to_user_id(&args[0])
                                            {
                                                Ok(uid) => { uid }
                                                Err(err) => { return Err(err) }
                                            };

                                            let recipient_key: PublicKey = match PublicKey::from_bytes(parsed_uid.as_ref())
                                            {
                                                Ok(pk) => { pk }
                                                Err(_) => { return Err(String::from("Please enter a valid public key for the recipient of the transfer")) }
                                            };
                                            let amount: u32 = match args[1].parse()
                                            {
                                                Ok(number) => { number }
                                                Err(_) => { return Err(String::from("Please enter a valid 32 bit unsigned integer for the amount of the transfer!")) }
                                            };

                                            Ok(Input::Transfer { sender: key.public.to_bytes(), recipient: recipient_key.to_bytes(), amount })
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
                                                    return Ok(Input::Balance { user: key.public.to_bytes() })
                                                }
                                        }
                                    }
                                1 =>
                                    {
                                        match string_to_user_id(&args[0])
                                        {
                                            Ok(uid) => { Ok(Input::Balance { user: uid }) }
                                            Err(err) => { Err(err) }
                                        }
                                    }

                                _ =>
                                    {
                                        return Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                                    }
                            }
                        }

                    "ldwallet" =>
                        {
                            if args.len() != 1
                            {
                                Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                            } else {
                                Ok(Input::LoadWallet { path: args[0].clone() })
                            }
                        }

                    "genwallet" =>
                        {
                            if args.len() != 1
                            {
                                Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                            }
                            else
                            {
                                Ok(Input::GenWallet { path: args[0].clone() })
                            }
                        }
                    "" =>
                        {
                            Ok(Input::None)
                        }

                    _ =>
                        {
                            Err(String::from("The typed command could not be recognised! (Type \"help\" to get a list of possible commands)"))
                        }
                }
            }
    }
}

