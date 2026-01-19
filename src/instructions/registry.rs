use crate::{
    helper::AnchorDiscriminator,
    instructions::{
        error::Error, events::Instruction, pump::instructions::{PumpBuyInstruction, PumpCreateInstruction, PumpSellInstruction}, raw_instruction::RawInstruction
    },
};

type Parser = fn(&[u8]) -> Result<Instruction, Error>;

pub struct ProgramData {
    pub program : Program,
    pub instructions : &'static [Decoder]
}

pub struct Decoder {
    pub disc: &'static [u8],
    pub parse: Parser,
}

impl Decoder {
    const fn new(disc : &'static [u8], parse : Parser) -> Self {
        Decoder { disc, parse }
    }
}

pub enum Program {
    PumpFun,
    BonkFun,
    Bags,
    ComputeBudget
}

pub const REGISTRY: &[ProgramData] = &[
    ProgramData {
        program : Program::PumpFun,

        instructions : &[
            Decoder::new(PumpCreateInstruction::DISCRIMINATOR, PumpCreateInstruction::instruction),
            Decoder::new(PumpBuyInstruction::DISCRIMINATOR, PumpBuyInstruction::instruction),
            Decoder::new(PumpSellInstruction::DISCRIMINATOR, PumpSellInstruction::instruction),
        ],
    },

    ProgramData {
        program : Program::ComputeBudget,

        instructions : &[
            Decoder::new(PumpCreateInstruction::DISCRIMINATOR, PumpCreateInstruction::instruction),
            Decoder::new(PumpBuyInstruction::DISCRIMINATOR, PumpBuyInstruction::instruction),
        ]
    }
];
