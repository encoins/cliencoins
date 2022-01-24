extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::client::Client;
use crate::instructions::Instruction;
use crate::message::Message;
use serde::{Serialize};

#[derive(Clone,Serialize)]
pub struct SignedInstruction {
    instruction : Instruction,
    signature : Vec<u8> // vec of (signature .to_byte (easier to serialize))
}


impl Instruction {

    pub fn sign_instruction(self, secret_key : &Keypair) -> SignedInstruction {
        let instruction : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(instruction).to_bytes().to_vec();
        SignedInstruction {
            instruction : self,
            signature
        }
    }
}

