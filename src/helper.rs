use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Link(pub String);
#[derive(Serialize, Deserialize)]
pub struct Base58Pubkey(pub String);
#[derive(BorshDeserialize, Debug)]
pub struct RawPubkey(pub [u8; 32]);

pub const DISCRIMINATOR_SIZE: usize = 8;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AnchorDiscriminator(pub [u8; DISCRIMINATOR_SIZE]);
#[derive(Serialize, Deserialize, BorshDeserialize, Debug)]
pub struct Amount(pub u64);

#[derive(Serialize, Deserialize, BorshDeserialize)]
pub struct Time(pub u64);
