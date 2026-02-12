use crate::helper::RawPubkey;

pub trait IntoAccountMetaArray {
    fn accounts_meta(self) -> alloc::vec::Vec<AccountMeta>;
}

#[derive(Debug)]
pub struct AccountMeta {
    pub pubkey: RawPubkey,
    pub is_signer: bool,
    pub writable: bool,
}
