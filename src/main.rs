use sips::{
    address::Address,
    helper::{Amount, Link},
    instructions::{
        compute_budget::{ComputeBudgetInstruction, ComputeUnitPrice},
        pump::instructions::{PumpCreateV2Instruction, PumpInstruction, PumpMetadata},
    },
};

fn main() {
    let program = Address::from_str_const("ComputeBudget111111111111111111111111111111");
    let shiet = sips::address::create_program_address(&[], &program).unwrap();
    let shiet2 = solana_pubkey::Pubkey::create_program_address(&[], &program.into()).unwrap();
    dbg!(shiet);
    dbg!(shiet2.as_array());
}
