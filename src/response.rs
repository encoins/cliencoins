use crate::base_types::Currency;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum Response {
    Balance(Currency),
    Transfer(bool,u8),
    SendErr,
    RcvErr
}

impl Response {

    pub fn to_string(self) -> String
    {
        match self
        {
            Response::Balance(amount) => { format!("Balance of {} ENcoinS",amount) }
            Response::Transfer(true,_) => { format!("Transfer succeeded") }
            Response::Transfer(false,1) => { format!("The server refused to start the transfer because the signature is not correct") }
            Response::Transfer(false,2) => { format!("The server refused to start the transfer because you don't have enough money on your account") }
            Response::Transfer(false,3) => { format!("The server refused to start a new transfer because it have not validated your previous one") }
            Response::Transfer(false,_) => { format!("Undetermined error") }
            Response::SendErr => {format!("Could not connect to a server!")}
            Response::RcvErr => {format!("Could not receive the response from the server!")}

        }
    }
}
