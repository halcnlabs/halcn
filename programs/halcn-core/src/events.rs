use anchor_lang::prelude::*;

#[event]
pub struct SignalDetected {
    pub signal: Pubkey,
    pub authority: Pubkey,
    pub source_market: String,
    pub threshold: u64,
    pub window_ms: u64,
    pub timestamp: i64,
}

#[event]
pub struct PropagationComputed {
    pub propagation_path: Pubkey,
    pub signal: Pubkey,
    pub authority: Pubkey,
    pub hop_count: u8,
    pub total_latency_ms: u64,
    pub timestamp: i64,
}

#[event]
pub struct ImpactPredicted {
    pub prediction: Pubkey,
    pub propagation_path: Pubkey,
    pub authority: Pubkey,
    pub magnitude_bps: u64,
    pub confidence_bps: u64,
    pub eta_ms: u64,
    pub timestamp: i64,
}

#[event]
pub struct PreArrivalAlert {
    pub prediction: Pubkey,
    pub target_market: String,
    pub magnitude_bps: u64,
    pub eta_ms: u64,
    pub timestamp: i64,
}
