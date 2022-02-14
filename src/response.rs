use crate::base_types::Currency;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Response {
    Balance(Currency),
    Transfer(bool,u8)
}

impl Response {
    pub fn print(self) {
        match self {
            Response::Balance(amount) => { println!("Balance of {} ENcoinS)",amount)}
            Response::Transfer(true,_) => { println!("Transfert succeeded") }
            Response::Transfer(false,1) => { println!("I don't have enough money to make this transfer! I won't even try to broadcast anything...") }
            Response::Transfer(false,2) => { println!("I have not validated your previous transfer yet") }
            Response::Transfer(false,_) => { println!("Arthur is a BIG noob")}
        }
    }
}
