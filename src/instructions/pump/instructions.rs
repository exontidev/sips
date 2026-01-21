use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use crate::{
    helper::{AccountIndex, Amount, Link, NATIVE_SOL_PRECISION, RawPubkey},
    instructions::{
        error::Error,
        instructions::Instruction,
        pump::accounts::{CreateAccount, CreateV2Account, TradeAccount},
        raw_instruction::{InstructionAccounts, RawSerializable},
    },
};

const PUMP_SPL_PRECISION: u8 = 6;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct PumpMetadata {
    name: String,
    symbol: String,
    uri: Link,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpCreateV2Instruction {
    metadata: PumpMetadata,
    creator: RawPubkey,
    mayhem: bool,
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpBuyInstruction {
    spl_amount: Amount<PUMP_SPL_PRECISION>,
    maximum_sol_input: Amount<PUMP_SPL_PRECISION>,
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpSellInstruction {
    spl_amount: Amount<PUMP_SPL_PRECISION>,
    minimum_sol_payout: Amount<NATIVE_SOL_PRECISION>,
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpBuyExactSolInInstruction {
    sol_amount: Amount<NATIVE_SOL_PRECISION>,
    minimum_token_output: Amount<PUMP_SPL_PRECISION>,
}

impl PumpBuyExactSolInInstruction {
    pub fn instruction(data: &[u8]) -> Result<Instruction, Error> {
        let ix = Self::from_bytes(data)?;
        Ok(Instruction::PumpBuyExactIn(ix))
    }
}

impl RawSerializable for PumpBuyExactSolInInstruction {
    const DISCRIMINATOR: &[u8] = &[56, 252, 116, 8, 158, 223, 205, 95];
}

impl InstructionAccounts for PumpBuyExactSolInInstruction {
    type Account = TradeAccount;
}
