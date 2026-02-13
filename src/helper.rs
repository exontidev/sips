use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Link(pub alloc::string::String);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RawPubkey(pub [u8; 32]);
impl RawPubkey {
    pub const fn from_str_const(data: &'static str) -> Self {
        RawPubkey(five8_const::decode_32_const(data))
    }
}
pub const DISCRIMINATOR_SIZE: usize = 8;
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AnchorDiscriminator(pub [u8; DISCRIMINATOR_SIZE]);

#[derive(BorshDeserialize)]
pub struct Time(pub u64);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Amount<const P: u8>(pub u64);

impl<const P: u8> Amount<P> {
    pub const SCALE: u64 = 10u64.pow(P as u32);

    pub fn from_float(v: f64) -> Self {
        Self((v * Self::SCALE as f64).round() as u64)
    }

    pub fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }

    pub fn to_float(&self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }
}

pub const NATIVE_SOL_PRECISION: u8 = 9;
