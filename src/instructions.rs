use std::fmt::{Display, Formatter};
use crate::base_types::{Currency, UserId};
use serde::{Serialize};

#[derive(Clone,Serialize)]
pub enum Instruction {

    Transfer{sender : UserId, recipient : UserId, amount : Currency},

    Balance{user: UserId}
}



impl Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Instruction::Balance {..} => { write!(f, " Balances") }
            Instruction::Transfer {sender,recipient,amount } => { write!(f, "New transfer : (sender : {}, recipient :{}, amount {})", sender, recipient, amount) }

        }
    }
}
