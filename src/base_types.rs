//! Definition of global types used in the algorithm

use serde::{Serialize};

/// A UserId is the user's public key
pub type UserId = ComprPubKey;

/// For the moment, the currency is encoded in a 32-bit integer. Defining how to deal with currency is still to be determined
pub type Currency = u32;

pub type ComprPubKey = [u8; 32]; // from CompressedEdwardsY

/// A transaction is an exchange of money between two users
#[derive(Clone, Serialize,Debug)]
pub struct Transfer
{
    pub sender : UserId,
    pub recipient : UserId,
    pub amount : Currency
}
