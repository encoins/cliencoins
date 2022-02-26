use ed25519_dalek::{Keypair, Signer};
use crate::base_types::{Transfer};
use crate::instructions::Instruction;

impl Transfer {

    /// Signs a transfer and transforms it into an instruction to be sent to the network
    pub fn sign(self, secret_key : &Keypair) -> Instruction {
        let transfer : &[u8] = &(bincode::serialize(&self).unwrap()[..]);
        let signature = secret_key.sign(transfer).to_bytes().to_vec();
        Instruction::SignedTransfer {
            transfer : self,
            signature
        }
    }
}


