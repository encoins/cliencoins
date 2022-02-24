use std::fmt::{Display, Formatter};
use ed25519_dalek::PublicKey;
use crate::base_types::{Currency, Transfer, UserId};
use serde::Serialize;


#[derive(Clone,Serialize,Debug)]
pub enum Instruction {
    SignedTransfer
    {
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },

    Balance{user: PublicKey}
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {user} => { write!(f, " Balances of {}", user.to_bytes().to_str().unwrap()) }
            Instruction::SignedTransfer {transfer, signature} => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})",transfer.sender.to_bytes().to_str().unwrap() , transfer.recipient.to_bytes().to_str().unwrap(), transfer.amount) }

        }
    }
}
