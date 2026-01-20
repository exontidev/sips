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

pub trait AccountIndex {
    fn index(self) -> usize;
}

impl AccountIndex for () {
    fn index(self) -> usize {
        unreachable!("This instruction has no accounts")
    }
}

#[macro_export]
macro_rules! impl_index {
    ($t:ty) => {
        impl AccountIndex for $t {
            fn index(self) -> usize {
                self as usize
            }
        }
    };
}