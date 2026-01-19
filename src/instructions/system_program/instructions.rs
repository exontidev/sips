use borsh::BorshDeserialize;

use crate::instructions::{error::Error, events::Instruction, raw_instruction::RawInstruction};

#[derive(BorshDeserialize, Debug)]
pub struct ComputeUnitLimit {
    limit : u32
}

impl ComputeUnitLimit {
    pub fn instruction(data : &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputeLimit(ix)) 
    }
}

impl RawInstruction<ComputeUnitLimit> for ComputeUnitLimit {
    const DISCRIMINATOR : &'static [u8] = &[2];
}

#[derive(BorshDeserialize, Debug)]
pub struct ComputeUnitPrice {
    limit : u64
}

impl ComputeUnitPrice {
    pub fn instruction(data : &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::ComputePrice(ix)) 
    }
}

impl RawInstruction<ComputeUnitPrice> for ComputeUnitPrice {
    const DISCRIMINATOR : &'static [u8] = &[3];
}