use crate::{
    helper::{AnchorDiscriminator, DISCRIMINATOR_SIZE},
    instructions::{
        error::Error,
        pump::instructions::{PumpBuyInstruction, PumpCreateInstruction, PumpSellInstruction},
        registry::REGISTRY, system_program::instructions::{ComputeUnitLimit, ComputeUnitPrice},
    },
};

pub enum Instruction {
    ComputeLimit(ComputeUnitLimit),
    ComputePrice(ComputeUnitPrice),
    PumpCreate(PumpCreateInstruction),
    PumpBuy(PumpBuyInstruction),
    PumpSell(PumpSellInstruction),
}

impl Instruction {
    pub fn from_bytes(ix: &[u8]) -> Result<Instruction, Error> {
        for entry in REGISTRY {
            for instruction in entry.instructions{
                return (instruction.parse)(ix);
            }
        }

        Err(Error::InvalidDiscriminator)
    }
}