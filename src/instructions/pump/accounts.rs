use ix_macros::Accounts;
use crate::helper::AccountIndex;


#[repr(usize)]
#[derive(Accounts)]
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