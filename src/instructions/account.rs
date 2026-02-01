use crate::helper::RawPubkey;

pub trait IntoAccountMetaArray<'a, const N: usize> {
    fn accounts_meta(&'a self) -> [AccountMeta<'a>; N];
}

pub struct AccountMeta<'a> {
    pub pubkey: &'a RawPubkey,
    pub is_signer: bool,
    pub writable: bool,
}
