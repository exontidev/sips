use sips::{
    helper::{Amount, Link, RawPubkey},
    instructions::{
        compute_budget::{ComputeBudgetInstruction, ComputeUnitPrice},
        pump::instructions::{PumpCreateV2Instruction, PumpInstruction, PumpMetadata},
        raw_instruction::{Instruction, InstructionWithAccounts},
    },
};

fn main() {
    let priority_fee = ComputeBudgetInstruction::priority_fee(100_000, Amount::from_float(0.1));

    let (price_ix, limit_ix): (
        solana_instruction::Instruction,
        solana_instruction::Instruction,
    ) = (
        priority_fee.price_ix.raw().into(),
        priority_fee.limit_ix.raw().into(),
    );

    dbg!(price_ix);
    dbg!(limit_ix);
}
