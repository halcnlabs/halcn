use anchor_lang::prelude::*;

use crate::constants::*;

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

#[account]
#[derive(Default)]
pub struct PropagationPath {
    pub signal: Pubkey,
    pub authority: Pubkey,
    pub path_nodes: Vec<String>,
    pub edge_weights: Vec<u64>,
    pub decay_factors: Vec<u64>,
    pub total_latency_ms: u64,
    pub hop_count: u8,
    pub computed_at: i64,
    pub bump: u8,
}

impl PropagationPath {
    pub const SIZE: usize = 8 + 32 + 32
        + 4 + (MAX_PATH_HOPS * (4 + MAX_MARKET_LEN))
        + 4 + (MAX_PATH_HOPS * 8)
        + 4 + (MAX_PATH_HOPS * 8)
        + 8 + 1 + 8 + 1;
}

#[account]
#[derive(Default)]
pub struct ImpactPrediction {
    pub propagation_path: Pubkey,
    pub authority: Pubkey,
    pub predicted_magnitude_bps: u64,
    pub confidence_lower_bps: u64,
    pub confidence_upper_bps: u64,
    pub confidence_level_bps: u64,
    pub eta_ms: u64,
    pub predicted_at: i64,
    pub alert_emitted: bool,
    pub target_market: String,
    pub bump: u8,
}

impl ImpactPrediction {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 4 + MAX_MARKET_LEN + 1;
}
