use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Instruction, Instructions};

use crate::{
    helper::{Amount, Link, NATIVE_SOL_PRECISION, RawPubkey},
    instructions::{
        error::Error,
        pump::accounts::{CreateAccounts, CreateV2Accounts, TradeAccounts},
        raw_instruction::{Instruction, InstructionArgs, RawInstruction},
    },
};

const PUMP_SPL_PRECISION: u8 = 6;

#[derive(Debug)]
pub enum PumpInstruction {
    Create(Instruction<{ CreateAccounts::ACCOUNT_LENGTH }, PumpCreateInstruction, CreateAccounts>),
    CreateV2(
        Instruction<
            { CreateV2Accounts::ACCOUNT_LENGTH },
            PumpCreateV2Instruction,
            CreateV2Accounts,
        >,
    ),

    Buy(Instruction<{ TradeAccounts::ACCOUNT_LENGTH }, PumpBuyInstruction, TradeAccounts>),
    BuyExactIn(
        Instruction<{ TradeAccounts::ACCOUNT_LENGTH }, PumpBuyExactSolInInstruction, TradeAccounts>,
    ),

    Sell(Instruction<{ TradeAccounts::ACCOUNT_LENGTH }, PumpSellInstruction, TradeAccounts>),
}

// impl PumpInstruction {
//     pub fn to_bytes(&self) -> RawInstruction<_> {
//         match self {
//             Self::Create(ix) => ix.into_raw(),
//             Self::CreateV2(ix) => todo!(),
//             Self::Buy(ix) => ix.into_raw(),
//             Self::BuyExactIn(ix) => ix.into_raw(),
//             Self::Sell(ix) => ix.into_raw(),
//         }
//     }
// }

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
