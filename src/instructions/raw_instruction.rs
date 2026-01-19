use borsh::{BorshDeserialize, from_slice};

use crate::instructions::error::Error;

pub trait RawInstruction<T : Sized + BorshDeserialize> {
    const DISCRIMINATOR : &'static [u8];

    fn from_bytes(data : &[u8]) -> Result<T, Error> {
        let discriminator_size = Self::DISCRIMINATOR.len();

        let data: &[u8] = data
            .get(discriminator_size..)
            .ok_or(Error::InvalidInstructionSize)?;

        let instruction: T = from_slice::<T>(&data)
            .map_err(|_| Error::InvalidInstructionData)?;

        Ok(instruction)
    }
}