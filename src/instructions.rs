use std::fmt::{Display, Formatter};
use ed25519_dalek::PublicKey;
use crate::base_types::{Currency, Transfer, UserId};
use serde::Serialize;
use crate::utils::user_id_to_string;


#[derive(Clone,Serialize,Debug)]
pub enum Instruction {
    SignedTransfer
    {
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },

    Balance{user: UserId}
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {user} => { write!(f, " Balances of {}", user_id_to_string(user)) }
            Instruction::SignedTransfer {transfer, signature} => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})", user_id_to_string( &transfer.sender) , user_id_to_string(&transfer.recipient), transfer.amount) }

        }
    }
}
