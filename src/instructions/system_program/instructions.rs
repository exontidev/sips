use borsh::{BorshDeserialize, BorshSerialize};

use crate::instructions::{
    error::Error, instructions::Instruction, raw_instruction::RawSerializable,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ComputeUnitLimit {
    limit: u32,
}

impl ComputeUnitLimit {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputeLimit(ix))
    }
}

impl RawSerializable for ComputeUnitLimit {
    const DISCRIMINATOR: &'static [u8] = &[2];
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ComputeUnitPrice {
    limit: u64,
}

impl ComputeUnitPrice {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputePrice(ix))
    }
}

impl RawSerializable for ComputeUnitPrice {
    const DISCRIMINATOR: &'static [u8] = &[3];
}
