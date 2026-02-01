use crate::helper::RawPubkey;

use crate::{
    helper::{Amount, NATIVE_SOL_PRECISION},
    instructions::raw_instruction::Instruction,
};
use borsh::{BorshDeserialize, BorshSerialize};
use ix_macros::{Accounts, Instruction};

use crate::instructions::account::{AccountMeta, IntoAccountMetaArray};

#[derive(Instruction, BorshSerialize, BorshDeserialize, Debug)]
#[ix_data(
    discriminator = [2, 0, 0, 0]
)]
pub struct Transfer {
    sol: Amount<NATIVE_SOL_PRECISION>,
}

#[derive(Accounts)]
pub struct TransferAccounts {
    #[signer]
    #[writable]
    pub sender: RawPubkey,
    #[writable]
    pub receiver: RawPubkey,
}
