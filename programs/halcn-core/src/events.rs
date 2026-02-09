use anchor_lang::prelude::*;

/// Emitted when a propagation path is computed.
#[event]
pub struct PropagationComputed {
    pub propagation_path: Pubkey,
    pub signal: Pubkey,
    pub authority: Pubkey,
    pub hop_count: u8,
    pub total_latency_ms: u64,
    pub timestamp: i64,
}
