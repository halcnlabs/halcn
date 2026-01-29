use anchor_lang::prelude::*;

use crate::constants::*;

/// Represents a detected signal event from a source market.
#[account]
#[derive(Default)]
pub struct SignalAccount {
    pub authority: Pubkey,
    pub source_market: String,
    pub threshold: u64,
    pub window_ms: u64,
    pub detected_at: i64,
    pub consumed: bool,
    pub signal_index: u64,
    pub bump: u8,
}

impl SignalAccount {
    pub const SIZE: usize = 8 + 32 + 4 + MAX_MARKET_LEN + 8 + 8 + 8 + 1 + 8 + 1;
}
