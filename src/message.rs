use ed25519_dalek::Signature;
use crate::base_types::ComprPubKey;
use crate::instructions::Instruction;

pub struct Message {
    id_sender : ComprPubKey,
    instruction : Instruction,
    signature : Signature
}