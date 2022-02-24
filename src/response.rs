use crate::base_types::Currency;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Response {
    Balance(Currency),
    Transfer(bool)
}

impl Response {
    pub fn print(self)
    {
        match self
        {
            Response::Balance(amount) => { println!("Balance of {} ENcoinS)",amount) }
            Response::Transfer(true) => { println!("Transfer succeeded") }
            Response::Transfer(false) => { println!("Not enough money to this transfer") }
        }
    }
}
