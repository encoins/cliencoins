//! Definition of the input enum defining regular inputs

use crate::base_types::{Currency, UserId};

/// An input can be either a request to make two Processus interact or to interact with the GUI
pub enum Input
{

    Transfer{ sender : UserId, recipient: UserId, amount : Currency },
    /// Input to get transactions history of an account according to a given account
    Help,
    /// Input to clear terminal from previous inputs
    Clear,
    /// Input to quit program
    Quit,
    Balance { user : UserId }
}

impl Input
{
    const WRONG_AMOUNT_OF_ARGS: &'static str = "Wrong amount of arguments! (Type \"help\" to see how to use command)";

    pub fn from(value: &Vec<&str>) -> Result<Input,String>
    {
        let mut args = vec![];
        return if value.len() == 0
        {
            Err(String::from("No command entered! Type \"help\" to get a list of possible commands"))
        } else {
            for k in 1..value.len()
            {
                let word = String::from(value[k]);
                let arg: u32 = match word.trim().parse()
                {
                    Ok(num) => num,
                    Err(_) =>
                        {
                            return Err(String::from("Arguments should be non negative numbers! (Type \"help\" to see how to use command)"));
                        }
                };
                args.push(arg);
            }

            // Transforms first argument to lowercase
            let value_lc = &value[0].to_lowercase()[..];

            match value_lc
            {


                "transfer" =>
                    {
                        if args.len() != 3
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Transfer { sender: args[0], recipient: args[1], amount: args[2] })
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
                        if args.len() != 1
                        {
                            Err(String::from(Input::WRONG_AMOUNT_OF_ARGS))
                        } else {
                            Ok(Input::Balance { user: args[0] })
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