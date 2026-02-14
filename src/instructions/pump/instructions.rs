use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Instruction, Instructions};

use crate::{
    helper::{Amount, Link, NATIVE_SOL_PRECISION, RawPubkey},
    instructions::{
        error::Error,
        pump::accounts::{
            CloseUserVolumeAccumulatorAccounts, CreateAccounts, CreateV2Accounts, TradeAccounts,
        },
        raw_instruction::{Instruction, InstructionArgs, ProgramAddress, RawInstruction},
    },
};

const PUMP_SPL_PRECISION: u8 = 6;

#[derive(Instructions, Debug)]
#[program("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P")]
pub enum PumpInstruction {
    Create(Instruction<PumpCreateInstruction, CreateAccounts>),
    CreateV2(Instruction<PumpCreateV2Instruction, CreateV2Accounts>),

    Buy(Instruction<PumpBuyInstruction, TradeAccounts>),
    BuyExactIn(Instruction<PumpBuyExactSolInInstruction, TradeAccounts>),

    Sell(Instruction<PumpSellInstruction, TradeAccounts>),
    CloseAccumulatorAccount(
        Instruction<CloseUserVolumeAccumulator, CloseUserVolumeAccumulatorAccounts>,
    ),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PumpMetadata {
    pub name: alloc::string::String,
    pub symbol: alloc::string::String,
    pub uri: Link,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [24, 30, 200, 40, 5, 28, 7, 119])]
pub struct PumpCreateInstruction {
    pub metadata: PumpMetadata,
    pub creator: RawPubkey,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [214, 144, 76, 236, 95, 139, 49, 180])]
pub struct PumpCreateV2Instruction {
    pub metadata: PumpMetadata,
    pub creator: RawPubkey,
    pub mayhem: bool,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [102, 6, 61, 18, 1, 218, 235, 234])]
pub struct PumpBuyInstruction {
    pub spl_amount: Amount<PUMP_SPL_PRECISION>,
    pub maximum_sol_input: Amount<NATIVE_SOL_PRECISION>,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [51, 230, 133, 164, 1, 127, 131, 173])]
pub struct PumpSellInstruction {
    pub spl_amount: Amount<PUMP_SPL_PRECISION>,
    pub minimum_sol_payout: Amount<NATIVE_SOL_PRECISION>,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [56, 252, 116, 8, 158, 223, 205, 95])]
pub struct PumpBuyExactSolInInstruction {
    pub sol_amount: Amount<NATIVE_SOL_PRECISION>,
    pub minimum_token_output: Amount<PUMP_SPL_PRECISION>,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [0xf9, 0x45, 0xa4, 0xda, 0x96, 0x67, 0x54, 0x8a])]
pub struct CloseUserVolumeAccumulator;
