use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, Serialize, Deserialize)]
pub struct Link(pub String);
#[derive(Serialize, Deserialize)]
pub struct Base58Pubkey(pub String);
pub struct RawPubkey(pub [u8; 32]);

pub const DISCRIMINATOR_SIZE: usize = 8;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AnchorDiscriminator(pub [u8; DISCRIMINATOR_SIZE]);
pub struct Amount(pub u64);
pub struct Time(pub u64);