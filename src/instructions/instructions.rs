use crate::instructions::{
    error::Error,
    pump::instructions::{
        PumpBuyInstruction, PumpCreateInstruction, PumpCreateV2Instruction, PumpSellInstruction,
    },
    registry::REGISTRY,
    system_program::instructions::{ComputeUnitLimit, ComputeUnitPrice},
};

#[derive(Debug)]
pub enum Instruction {
    ComputeLimit(ComputeUnitLimit),
    ComputePrice(ComputeUnitPrice),

    PumpCreate(PumpCreateInstruction),
    PumpCreateV2(PumpCreateV2Instruction),
    PumpBuy(PumpBuyInstruction),
    PumpSell(PumpSellInstruction),
}

impl Instruction {
    pub fn from_bytes(ix: &[u8]) -> Result<Instruction, Error> {
        for entry in REGISTRY {
            for instruction in entry.instructions {
                let Ok(instruction) = (instruction.parse)(ix) else {
                    continue;
                };

                return Ok(instruction);
            }
        }

        Err(Error::InvalidDiscriminator)
    }
}
