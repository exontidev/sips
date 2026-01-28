use crate::{helper::RawPubkey, instructions::account::{AccountMeta, Accounts}};
use ix_macros::Accounts;

#[repr(usize)]
#[derive(Accounts)]
pub enum CreateAccount {
    #[signer]
    #[writable]
    Mint,
    MintAuthority,

    #[writable]
    BondingCurve,

    #[writable]
    AssociatedBondingCurve,

    Global,
    MetaplexTokenMetadata,

    #[writable]
    Metadata,

    #[signer]
    #[writable]
    User,

    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    Rent,
    EventAuthority,
    Program,
}

#[repr(usize)]
#[derive(Accounts)]
pub enum CreateV2Account {
    Mint,
    MintAuthority,
    BondingCurve,
    AssociatedBondingCurve,
    Global,
    User,
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    MayhemProgram,
    GlobalParams,
    SolVault,
    MayhemState,
    MayhemTokenVault,
    EventAuthority,
}

#[repr(usize)]
#[derive(Accounts)]
pub enum TradeAccount {
    Global,
    FeeAddress,
    Mint,
    BondingCurve,
    AssociatedBondingCurve,
    AssociatedUser,
    SystemProgram,
    TokenProgram,
    CreatorVault,
    EventAuthority,
    Program,
    GlobalVolumeAccumulator,
    UserVolumeAccumulator,
    FeeConfig,
}
