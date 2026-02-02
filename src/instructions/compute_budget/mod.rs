use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Instruction, Instructions};

use crate::instructions::{error::Error, raw_instruction::InstructionArgs};

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [2])]
pub struct ComputeUnitLimit {
    limit: u32,
}

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [3])]
pub struct ComputeUnitPrice {
    limit: u64,
}

#[derive(Instructions, Debug)]
pub enum ComputeBudgetInstruction {
    SetUnitPrice(ComputeUnitPrice),
    SetComputeLimit(ComputeUnitLimit),
}
