use crate::{helper::AccountIndex, impl_index};

#[repr(usize)]
pub enum CreateAccount {
    Mint,
    MintAuthority,
    BondingCurve,
    AssociatedBondingCurve,
    Global,
    MetaplexTokenMetadata,
    Metadata,
    User,
    SystemProgram,
    TokenProgram,
    AssociatedTokenProgram,
    Rent,
    EventAuthority,
    Program,
}

#[repr(usize)]
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

impl_index!(CreateAccount);
impl_index!(CreateV2Account);
impl_index!(TradeAccount);
