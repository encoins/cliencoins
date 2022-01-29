extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::client::Client;
use crate::instructions::{Instruction, Transfer};
use crate::message::Message;
use serde::{Serialize};

#[derive(Clone,Serialize)]
pub struct SignedTransfer {
    transfer : Transfer,
    signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
}


impl Transfer {

    pub fn sign_transfer(self, secret_key : &Keypair) -> SignedTransfer {
        let transfer : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(transfer).to_bytes().to_vec();
        SignedTransfer {
            transfer : self,
            signature
        }
    }
}

