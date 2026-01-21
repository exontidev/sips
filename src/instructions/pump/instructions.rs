use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::{
    helper::{AccountIndex, Link, RawPubkey},
    instructions::{error::Error, instructions::Instruction, pump::accounts::{CreateAccount, CreateV2Account, TradeAccount}, raw_instruction::{InstructionAccounts, RawSerializable}},
};

#[derive(BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct PumpMetadata {
    name: String,
    symbol: String,
    uri: Link,
}

#[derive(BorshDeserialize, Debug)]
pub struct PumpCreateInstruction {
    pub metadata: PumpMetadata,
    pub creator: RawPubkey,
}

impl PumpCreateInstruction {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpCreate(ix))
    }
}

impl RawSerializable for PumpCreateInstruction {
    const DISCRIMINATOR: &[u8] = &[24, 30, 200, 40, 5, 28, 7, 119];
}

impl InstructionAccounts for PumpCreateInstruction {
    type Account = CreateAccount;
}

#[derive(BorshDeserialize, Debug)]
pub struct PumpCreateV2Instruction {
    metadata: PumpMetadata,
    creator: RawPubkey,
    mayhem : bool
}

impl PumpCreateV2Instruction {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpCreateV2(ix))
    }
}

impl RawSerializable for PumpCreateV2Instruction {
    const DISCRIMINATOR: &[u8] = &[214, 144, 76, 236, 95, 139, 49, 180];
}

impl InstructionAccounts for PumpCreateV2Instruction {
    type Account = CreateV2Account;
}

#[derive(BorshDeserialize, Debug)]
pub struct PumpBuyInstruction {
    amount: u64,   // Token amount,
    slipapge: u64, // Max SOL payed
}

impl PumpBuyInstruction {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpBuy(ix))
    }
}

impl RawSerializable for PumpBuyInstruction {
    const DISCRIMINATOR: &[u8] = &[102, 6, 61, 18, 1, 218, 235, 234];
}

impl InstructionAccounts for PumpBuyInstruction {
    type Account = TradeAccount;
}

#[derive(BorshDeserialize, Debug)]
pub struct PumpSellInstruction {
    amount: u64,   // Token amount,
    slippage: u64, // Minimum SOL payout
}

impl PumpSellInstruction {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpSell(ix))
    }
}

impl RawSerializable for PumpSellInstruction {
    const DISCRIMINATOR: &[u8] = &[51, 230, 133, 164, 1, 127, 131, 173];
}

impl InstructionAccounts for PumpSellInstruction {
    type Account = TradeAccount;
}