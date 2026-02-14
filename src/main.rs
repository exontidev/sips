use sips::{
    helper::{Amount, Link, RawPubkey},
    instructions::{
        compute_budget::{ComputeBudgetInstruction, ComputeUnitPrice},
        pump::instructions::{PumpCreateV2Instruction, PumpInstruction, PumpMetadata},
    },
};

fn main() {
    let (price_ix, limit_ix) =
        ComputeBudgetInstruction::priority_fee(100_000, Amount::from_float(0.1));

    let (price_ix, limit_ix): (
        solana_instruction::Instruction,
        solana_instruction::Instruction,
    ) = (price_ix.into(), limit_ix.into());

    dbg!(price_ix);
    dbg!(limit_ix);
}
