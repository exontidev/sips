use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Instruction, Instructions};

use crate::{
    helper::{Amount, NATIVE_SOL_PRECISION, RawPubkey},
    instructions::{
        error::Error,
        raw_instruction::{Instruction, InstructionArgs},
    },
};

const COMPUTE_BUDGET_PUBKEY: RawPubkey = RawPubkey(five8_const::decode_32_const(
    "ComputeBudget111111111111111111111111111111",
));

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [2])]
pub struct ComputeUnitLimit {
    pub limit: u32,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [3])]
pub struct ComputeUnitPrice {
    pub limit: u128,
}

impl ComputeUnitPrice {
    pub fn from_sol(amount: Amount<NATIVE_SOL_PRECISION>) -> Self {
        Self {
            limit: amount.raw() as u128 * 1_000_000,
        }
    }
}

#[derive(Debug)]
pub enum ComputeBudgetInstruction {
    SetUnitPrice(Instruction<ComputeUnitPrice>),
    SetComputeLimit(Instruction<ComputeUnitLimit>),
}
