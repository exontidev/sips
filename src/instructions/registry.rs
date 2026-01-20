use crate::{
    helper::AnchorDiscriminator,
    instructions::{
        error::Error,
        instructions::Instruction,
        pump::instructions::{PumpBuyInstruction, PumpCreateInstruction, PumpCreateV2Instruction, PumpSellInstruction},
        raw_instruction::RawSerializable,
        system_program::instructions::{ComputeUnitLimit, ComputeUnitPrice},
    },
};

type Parser = fn(&[u8]) -> Result<Instruction, Error>;

pub struct ProgramData {
    pub program: Program,
    pub instructions: &'static [Decoder],
}

pub struct Decoder {
    pub disc: &'static [u8],
    pub parse: Parser,
}

impl Decoder {
    const fn new(disc: &'static [u8], parse: Parser) -> Self {
        Decoder { disc, parse }
    }
}

pub enum Program {
    PumpFun,
    BonkFun,
    Bags,
    ComputeBudget,
}

pub const REGISTRY: &[ProgramData] = &[
    ProgramData {
        program: Program::PumpFun,

        instructions: &[
            Decoder::new(
                PumpCreateInstruction::DISCRIMINATOR,
                PumpCreateInstruction::instruction,
            ),

            Decoder::new(
                PumpCreateV2Instruction::DISCRIMINATOR,
                PumpCreateV2Instruction::instruction,
            ),

            Decoder::new(
                PumpBuyInstruction::DISCRIMINATOR,
                PumpBuyInstruction::instruction,
            ),
            Decoder::new(
                PumpSellInstruction::DISCRIMINATOR,
                PumpSellInstruction::instruction,
            ),
        ],
    },
    ProgramData {
        program: Program::ComputeBudget,

        instructions: &[
            Decoder::new(
                ComputeUnitLimit::DISCRIMINATOR,
                ComputeUnitLimit::instruction,
            ),
            Decoder::new(
                ComputeUnitPrice::DISCRIMINATOR,
                ComputeUnitPrice::instruction,
            ),
        ],
    },
];
