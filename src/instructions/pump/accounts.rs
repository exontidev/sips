use crate::address::Address;
use crate::instructions::account::{AccountMeta, IntoAccountMetaArray};
use ix_macros::Accounts;

#[derive(Accounts, Debug)]
pub struct CreateAccounts {
    #[signer]
    #[writable]
    pub mint: Address,
    pub mint_authority: Address,

    #[writable]
    // #[seeds = b"bonding_curve" + mint]
    pub bonding_curve: Address,

    #[writable]
    pub associated_bonding_curve: Address,

    pub global: Address,
    pub metaplex_token_metadata_program: Address,

    #[writable]
    pub metadata: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub rent: Address,
    pub event_authority: Address,
    pub program: Address,
}

#[derive(Accounts, Debug)]
pub struct CreateV2Accounts {
    #[signer]
    #[writable]
    pub mint: Address,
    pub mint_authority: Address,

    #[writable]
    pub bonding_curve: Address,

    #[writable]
    pub associated_bonding_curve: Address,

    pub global: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,

    #[writable]
    pub mayhem_program: Address,

    pub global_params: Address,

    #[writable]
    pub sol_vault: Address,

    #[writable]
    pub mayhem_state: Address,

    #[writable]
    pub mayhem_token_vault: Address,

    pub event_authority: Address,
}

#[derive(Accounts, Debug)]
pub struct TradeAccounts {
    pub global: Address,
    #[writable]
    pub fee_address: Address,
    pub mint: Address,
    #[writable]
    pub bonding_curve: Address,
    #[writable]
    pub associated_bonding_curve: Address,
    #[writable]
    pub associated_user: Address,

    #[signer]
    #[writable]
    pub user: Address,

    pub system_program: Address,
    pub token_program: Address,

    #[writable]
    pub creator_vault: Address,
    pub event_authority: Address,
    pub program: Address,

    #[writable]
    pub global_volume_accumulator: Address,

    #[writable]
    pub user_volume_accumulator: Address,
    pub fee_config: Address,
}

#[derive(Accounts, Debug)]
pub struct CloseUserVolumeAccumulatorAccounts {
    #[signer]
    #[writable]
    user: Address,
    #[writable]
    user_volume_accumulator: Address,
    event_authority: Address,
    program: Address,
}
