extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use crate::crypto::ed25519_dalek::Signer;
use ed25519_dalek::{PublicKey, Verifier,Signature,Keypair};
use crate::message::Message;
