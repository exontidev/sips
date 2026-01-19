use serde::{Deserialize, Serialize};

use crate::{helper::{Amount, Base58Pubkey, Time}, instructions::pump::instructions::PumpMetadata};

#[derive(Serialize, Deserialize)]
pub struct CreateEvent {
    #[serde(flatten)]
    metadata : PumpMetadata,
    creator : Base58Pubkey
}

pub struct TradeEvent {
    is_buy : bool,
    sol_amount : Amount,
    token_amount : Amount, 
    user : Base58Pubkey,
    timestamp : Time,
    virtual_sol_reserves : Amount,
    virtual_token_reserves : Amount,
    token_creator : Base58Pubkey,
}