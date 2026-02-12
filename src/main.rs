use sips::{
    helper::{Amount, Link, RawPubkey},
    instructions::{
        compute_budget::ComputeUnitPrice,
        pump::instructions::{PumpCreateV2Instruction, PumpInstruction, PumpMetadata},
        raw_instruction::{Instruction, InstructionWithAccounts},
    },
};

fn main() {
    let create = PumpInstruction::CreateV2(InstructionWithAccounts {
        data: PumpCreateV2Instruction {
            metadata: PumpMetadata {
                name: "ObamaCoin".to_string(),
                symbol: "OBAMA".to_string(),
                uri: Link("https://shitfuck.org".to_string()),
            },

            creator: RawPubkey(five8_const::decode_32_const(
                "9G3oaisiANmLGS16iE4XimVyceqdnRbj8PoPs5RJSu7C",
            )),

            mayhem: false,
        },

        accounts: todo!(),
    });

    dbg!(buy);
}
