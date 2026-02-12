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
pub struct Instruction<Args: InstructionArgs> {
    pub data: Args,
}

impl<Args> Instruction<Args>
where
    Args: InstructionArgs,
{
    pub fn into_raw(self, program: RawPubkey) -> RawInstruction {
        let data = self.data.to_le_bytes();

        RawInstruction {
            program,
            data,
            accounts: alloc::vec![],
        }
    }
}

#[derive(Debug)]
pub struct InstructionWithAccounts<Args: InstructionArgs, Accounts: IntoAccountMetaArray> {
    pub data: Args,
    pub accounts: Accounts,
}

impl<Args, Accounts> InstructionWithAccounts<Args, Accounts>
where
    Args: InstructionArgs,
    Accounts: IntoAccountMetaArray,
{
    pub fn into_raw(self, program: RawPubkey) -> RawInstruction {
        let data = self.data.to_le_bytes();
        let accounts = self.accounts.accounts_meta();

        RawInstruction {
            program,
            data,
            accounts,
        }
    }
}

#[derive(Debug)]
pub struct RawInstruction {
    pub program: RawPubkey,
    pub data: Vec<u8>,
    pub accounts: Vec<AccountMeta>,
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
