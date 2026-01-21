use sip_rs::instructions::pump::accounts::{CreateAccount, CreateV2Account, TradeAccount};
use sip_rs::instructions::raw_instruction::InstructionAccounts;
use sip_rs::instructions::{Instruction, pump::instructions::PumpCreateInstruction};
fn main() {
    const ACCOUNTS: &[&str] = &[
        "Mint",
        "Mint Authority",
        "Bonding Curve",
        "Associated Bonding Curve",
        "Global",
        "Mpl Token Metadata",
        "Metadata",
        "User",
        "System Program",
        "Token Program",
        "Associated Token Program",
        "Rent",
        "Event Authority",
        "Program",
    ];

    let data: [u8; 24] = [
        0x38, 0xfc, 0x74, 0x08, 0x9e, 0xdf, 0xcd, 0x5f,
        0x80, 0xf0, 0xfa, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x23, 0xce, 0x85, 0xde, 0x66, 0x00, 0x00, 0x00
    ];


    let instruction = Instruction::from_bytes(&data).unwrap();

    match instruction {
        Instruction::PumpBuyExactIn(buy_ix) => {
            dbg!(&buy_ix);
            dbg!(&buy_ix.account_index(TradeAccount::Mint));
        },

        _ => ()
    }
}
