use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Accounts, Instruction};
use crate::helper::AccountIndex;
use crate::{helper::{Amount, NATIVE_SOL_PRECISION}, instructions::raw_instruction::RawSerializable};

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(discriminator = [2, 0, 0, 0])]
pub struct Transfer {
    sol: Amount<NATIVE_SOL_PRECISION>,
}

#[repr(u8)]
#[derive(Accounts)]
pub enum TransferAccount {
    Sender,
    Receiver
}