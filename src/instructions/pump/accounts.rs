use crate::helper::RawPubkey;
use crate::instructions::account::{AccountMeta, IntoAccountMetaArray};
use ix_macros::Accounts;

#[derive(Accounts, Debug)]
pub struct CreateAccounts {
    #[signer]
    #[writable]
    pub mint: RawPubkey,
    pub mint_authority: RawPubkey,

    #[writable]
    pub bonding_curve: RawPubkey,

    #[writable]
    pub associated_bonding_curve: RawPubkey,

    pub global: RawPubkey,
    pub metaplex_token_metadata_program: RawPubkey,

    #[writable]
    pub metadata: RawPubkey,

    #[signer]
    #[writable]
    pub user: RawPubkey,

    pub system_program: RawPubkey,
    pub token_program: RawPubkey,
    pub associated_token_program: RawPubkey,
    pub rent: RawPubkey,
    pub event_authority: RawPubkey,
    pub program: RawPubkey,
}

#[derive(Accounts, Debug)]
pub struct CreateV2Accounts {
    #[signer]
    #[writable]
    pub mint: RawPubkey,
    pub mint_authority: RawPubkey,

    #[writable]
    pub bonding_curve: RawPubkey,

    #[writable]
    pub associated_bonding_curve: RawPubkey,

    pub global: RawPubkey,

    #[signer]
    #[writable]
    pub user: RawPubkey,

    pub system_program: RawPubkey,
    pub token_program: RawPubkey,
    pub associated_token_program: RawPubkey,

    #[writable]
    pub mayhem_program: RawPubkey,

    pub global_params: RawPubkey,

    #[writable]
    pub sol_vault: RawPubkey,

    #[writable]
    pub mayhem_state: RawPubkey,

    #[writable]
    pub mayhem_token_vault: RawPubkey,

    pub event_authority: RawPubkey,
}

#[derive(Accounts, Debug)]
pub struct TradeAccounts {
    pub global: RawPubkey,
    #[writable]
    pub fee_address: RawPubkey,
    pub mint: RawPubkey,
    #[writable]
    pub bonding_curve: RawPubkey,
    #[writable]
    pub associated_bonding_curve: RawPubkey,
    #[writable]
    pub associated_user: RawPubkey,

    #[signer]
    #[writable]
    pub user: RawPubkey,

    pub system_program: RawPubkey,
    pub token_program: RawPubkey,

    #[writable]
    pub creator_vault: RawPubkey,
    pub event_authority: RawPubkey,
    pub program: RawPubkey,

    #[writable]
    pub global_volume_accumulator: RawPubkey,

    #[writable]
    pub user_volume_accumulator: RawPubkey,
    pub fee_config: RawPubkey,
}
