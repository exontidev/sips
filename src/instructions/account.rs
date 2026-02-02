use crate::helper::RawPubkey;

pub trait IntoAccountMetaArray<const N: usize> {
    fn accounts_meta(self) -> [AccountMeta; N];
}

pub struct AccountMeta {
    pub pubkey: RawPubkey,
    pub is_signer: bool,
    pub writable: bool,
}
