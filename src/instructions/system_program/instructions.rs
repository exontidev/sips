use borsh::BorshDeserialize;

use crate::instructions::{error::Error, instructions::Instruction, raw_instruction::RawSerializable};

#[derive(BorshDeserialize, Debug)]
pub struct ComputeUnitLimit {
    limit: u32,
}

impl ComputeUnitLimit {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputeLimit(ix))
    }
}

impl RawSerializable<ComputeUnitLimit> for ComputeUnitLimit {
    const DISCRIMINATOR: &'static [u8] = &[2];
}

#[derive(BorshDeserialize, Debug)]
pub struct ComputeUnitPrice {
    limit: u64,
}

impl ComputeUnitPrice {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputePrice(ix))
    }
}

impl RawSerializable<ComputeUnitPrice> for ComputeUnitPrice {
    const DISCRIMINATOR: &'static [u8] = &[3];
}
