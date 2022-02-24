extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::instructions::{Instruction};
use serde::{Serialize};


/*
impl Transfer {

    pub fn sign_transfer(self, secret_key : &Keypair) -> Instruction {
        let transfer : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(transfer).to_bytes().to_vec();
        Instruction::SignedTransfer {
            transfer : self,
            signature
        }
    }
}
*/


