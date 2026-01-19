use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::{
    helper::Link,
    instructions::{
        error::Error, events::Instruction, raw_instruction::RawInstruction
    },
};

#[derive(BorshDeserialize, Serialize, Deserialize)]
pub struct PumpMetadata {
    name: String,
    symbol: String,
    uri: Link,
}

#[derive(BorshDeserialize)]
pub struct PumpCreateInstruction {
    metadata: PumpMetadata
}

impl PumpCreateInstruction {
    pub fn instruction(data : &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpCreate(ix)) 
    }
}

impl RawInstruction<PumpCreateInstruction> for PumpCreateInstruction {
    const DISCRIMINATOR : &[u8] = &[24, 30, 200, 40, 5, 28, 7, 119];
}

#[derive(BorshDeserialize)]
pub struct PumpBuyInstruction {
    amount: u64,   // Token amount,
    slipapge: u64, // Max SOL payed
}

impl PumpBuyInstruction {
    pub fn instruction(data : &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpBuy(ix)) 
    }
}

impl RawInstruction<PumpBuyInstruction> for PumpBuyInstruction {
    const DISCRIMINATOR : &[u8] = &[102, 6, 61, 18, 1, 218, 235, 234];
}

#[derive(BorshDeserialize)]
pub struct PumpSellInstruction {
    amount: u64,   // Token amount,
    slippage: u64, // Minimum SOL payout
}

impl PumpSellInstruction {
    pub fn instruction(data : &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpSell(ix)) 
    }
}

impl RawInstruction<PumpSellInstruction> for PumpSellInstruction {
    const DISCRIMINATOR : &[u8] = &[51, 230, 133, 164, 1, 127, 131, 173];
}
