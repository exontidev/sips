use sips::{
    address::Address,
    helper::{Amount, Link},
    instructions::{
        compute_budget::{ComputeBudgetInstruction, ComputeUnitPrice},
        pump::instructions::{PumpCreateV2Instruction, PumpInstruction, PumpMetadata},
    },
};

fn main() {
    let program = Address::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
    // let shiet = sips::address::create_program_address(&[], &program).unwrap();

    let mint = Address::from_str_const("Cf324EQSX3b2LdUyJ1aR11qiMzWNRsixG6G2pduLpump").to_bytes();
    //let shiet2 = solana_pubkey::Pubkey::create_program_address(seeds, &program.into()).unwrap();

    let seeds: &[&[u8]] = &[b"bonding-curve", mint.as_slice()];
    let myaddress = Address::pda(&program, seeds);
    let program = Address::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
    let address = solana_pubkey::Pubkey::find_program_address(seeds, &program.into());

    dbg!(address.0);
    dbg!(myaddress.0.to_bytes());
}
