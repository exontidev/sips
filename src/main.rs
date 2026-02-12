use sips::{
    helper::{Amount, RawPubkey},
    instructions::{
        account::IntoAccountMetaArray,
        pump::{
            accounts::{CreateAccounts, TradeAccounts},
            instructions::{PumpInstruction, PumpSellInstruction},
        },
        raw_instruction::{Instruction, InstructionArgs},
        system_program::{Transfer, TransferAccounts},
    },
};

fn main() {
    let buy = PumpInstruction::Buy(Instruction {
        program: RawPubkey([
            79, 107, 26, 157, 62, 124, 47, 138, 28, 85, 208, 183, 164, 227, 242, 156, 109, 142,
            123, 42, 31, 12, 157, 62, 106, 75, 92, 125, 142, 159, 1, 0,
        ]),

        data: sips::instructions::pump::instructions::PumpBuyInstruction {
            spl_amount: Amount::from_float(100_000.0),
            maximum_sol_input: Amount::from_float(1.0),
        },

        accounts: todo!(),
    });
}
