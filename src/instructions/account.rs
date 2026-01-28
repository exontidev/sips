use crate::helper::RawPubkey;

pub trait Accounts {
    const ACCOUNT_LENGTH : usize;
    fn index(self) -> usize;
}

pub struct AccountMeta {
    pub pubkey: RawPubkey,
    pub is_signer: bool,
    pub writeble: bool,
}
