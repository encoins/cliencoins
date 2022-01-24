use crate::base_types::ComprPubKey;

use rand::rngs::OsRng;
use ed25519_dalek::{PublicKey,Verifier,Signature,Keypair};
use crate::message::Message;

pub struct Client {
    pub id : ComprPubKey,
    pub secret_key : Keypair
}

impl Client {
    pub fn new() -> Client {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        //let PublicKey(id,_) = keypair.public;
        Client {
            //id :id as ComprPubKey,
            id : keypair.public.to_bytes(),
            secret_key : keypair
        }
    }
}
