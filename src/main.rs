use sips::{
    helper::{Amount, RawPubkey},
    instructions::{compute_budget::ComputeUnitPrice, raw_instruction::Instruction},
};

fn main() {
    let buy = Instruction {
        program: RawPubkey(five8_const::decode_32_const(
            "ComputeBudget111111111111111111111111111111",
        )),
        data: ComputeUnitPrice::from_sol(Amount::from_float(0.5)),
    }
    .into_raw();

    dbg!(buy);
}
