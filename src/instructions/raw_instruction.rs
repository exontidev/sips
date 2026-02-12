use crate::{
    helper::RawPubkey,
    instructions::{
        account::{AccountMeta, IntoAccountMetaArray},
        error::Error,
    },
};

use alloc::vec::Vec;
use borsh::{BorshDeserialize, BorshSerialize, from_slice};

#[derive(Debug)]
pub struct Instruction<const N: usize, Args: InstructionArgs, Accounts: IntoAccountMetaArray<N>> {
    pub program: RawPubkey,
    pub data: Args,
    pub accounts: Accounts,
}

impl<const N: usize, Args, Accounts> Instruction<N, Args, Accounts>
where
    Args: InstructionArgs,
    Accounts: IntoAccountMetaArray<N>,
{
    pub fn into_raw(self) -> RawInstruction<N> {
        let data = self.data.to_le_bytes();
        let accounts = self.accounts.accounts_meta();
        let program = self.program;

        RawInstruction {
            program,
            data,
            accounts,
        }
    }
}

pub struct RawInstruction<const N: usize> {
    pub program: RawPubkey,
    pub data: Vec<u8>,
    pub accounts: [AccountMeta; N],
}

pub trait InstructionArgs: Sized + BorshSerialize + BorshDeserialize {
    const DISCRIMINATOR: &'static [u8];

    fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        let discriminator_size = Self::DISCRIMINATOR.len();

        let discriminator = data
            .get(..discriminator_size)
            .ok_or(Error::InstructionDataIsTooSmall)?;

        if Self::DISCRIMINATOR != discriminator {
            return Err(Error::InvalidDiscriminator);
        }

        let data: &[u8] = data
            .get(discriminator_size..)
            .ok_or(Error::InvalidInstructionSize)?;

        let instruction = from_slice::<Self>(&data).map_err(|_| Error::InvalidInstructionData)?;

        Ok(instruction)
    }

    fn to_le_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut data = alloc::vec::Vec::new();
        data.extend_from_slice(Self::DISCRIMINATOR);

        self.serialize(&mut data)
            .expect("Borsh serialization failed");

        data
    }
}
