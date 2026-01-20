
use borsh::BorshDeserialize;

use crate::{
    helper::{Amount, RawPubkey, Time}, instructions::PumpMetadata,
};

#[derive(BorshDeserialize)]
pub struct CreateEvent {
    metadata: PumpMetadata,

    mint : RawPubkey,
    bonding_curve : RawPubkey,
    user : RawPubkey,
    creator: RawPubkey,
    virtual_token_reserves : Amount,
    virtual_sol_reserves : Amount,
    token_program : RawPubkey,
    is_mayhem_mode : bool
}


#[derive(BorshDeserialize)]
pub struct TradeEvent {
    is_buy: bool,
    sol_amount: Amount,
    token_amount: Amount,
    user: RawPubkey,
    timestamp: Time,
    virtual_sol_reserves: Amount,
    virtual_token_reserves: Amount,
    token_creator: RawPubkey,
}
