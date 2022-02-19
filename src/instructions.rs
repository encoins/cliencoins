use std::fmt::{Display, Formatter};
use crate::base_types::{ComprPubKey, Currency, UserId};
use serde::{Serialize};

#[derive(Clone,Serialize,Debug)]
pub struct Transfer {
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}


#[derive(Clone,Serialize,Debug)]
pub enum Instruction {
    // redondance avec la def de crypto :(
    SignedTransfer {
        pub_key : ComprPubKey,
        transfer : Transfer,
        signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
    },

    Balance{user: UserId},
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {user} => { write!(f, " Balances of {}", user) }
            Instruction::SignedTransfer {pub_key,transfer, signature} => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})",transfer.sender , transfer.recipient, transfer.amount) }

        }
    }
}
