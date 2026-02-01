use sips::{
    helper::RawPubkey,
    instructions::{account::IntoAccountMetaArray, system_program::TransferAccounts},
};

fn main() {
    let alice: [u8; 32] = [
        18, 159, 18, 191, 18, 122, 30, 110, 66, 88, 129, 168, 99, 163, 147, 35, 23, 46, 99, 13,
        248, 129, 73, 51, 132, 226, 45, 213, 216, 122, 205, 163,
    ];

    let bob: [u8; 32] = [
        28, 209, 233, 102, 187, 252, 97, 169, 125, 81, 135, 193, 87, 191, 70, 135, 249, 139, 156,
        183, 140, 31, 212, 177, 222, 67, 111, 21, 18, 230, 53, 242,
    ];

    let transfer_accounts = TransferAccounts {
        sender: RawPubkey(alice),
        receiver: RawPubkey(bob),
    };
}
