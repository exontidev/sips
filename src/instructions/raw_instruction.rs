use crate::{helper::AccountIndex, instructions::error::Error};
use borsh::{BorshDeserialize, from_slice};

pub trait RawSerializable : Sized + BorshDeserialize {
    type Account: AccountIndex;

    const DISCRIMINATOR: &'static [u8];

    fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        let discriminator_size = Self::DISCRIMINATOR.len();

        let data: &[u8] = data
            .get(discriminator_size..)
            .ok_or(Error::InvalidInstructionSize)?;

        let instruction = from_slice::<Self>(&data).map_err(|_| Error::InvalidInstructionData)?;

        Ok(instruction)
    }

    fn account_index(&self, account : Self::Account) -> usize {
        account.index()
    }
}
