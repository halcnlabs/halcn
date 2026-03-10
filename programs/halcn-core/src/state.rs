use anchor_lang::prelude::*;

use crate::constants::*;

/// Represents a detected signal event from a source market.
#[account]
#[derive(Default)]
pub struct SignalAccount {
    /// Authority that created this signal record.
    pub authority: Pubkey,
    /// Source market identifier (e.g. "SOL/USDC").
    pub source_market: String,
    /// Detection threshold in basis points.
    pub threshold: u64,
    /// Sliding window size in milliseconds.
    pub window_ms: u64,
    /// Unix timestamp when the signal was detected.
    pub detected_at: i64,
    /// Whether this signal has been consumed by a propagation path.
    pub consumed: bool,
    /// Sequential signal index for ordering.
    pub signal_index: u64,
    /// Bump seed for PDA derivation.
    pub bump: u8,
}

impl SignalAccount {
    // discriminator + authority + source_market + threshold + window_ms
    //   + detected_at + consumed + signal_index + bump
    pub const SIZE: usize = 8 + 32 + (4 + MAX_MARKET_LEN) + 8 + 8 + 8 + 1 + 8 + 1;
}

/// Records the propagation path a signal takes across markets.
#[account]
#[derive(Default)]
pub struct PropagationPath {
    /// The signal account this path originates from.
    pub signal: Pubkey,
    /// Authority that computed this path.
    pub authority: Pubkey,
    /// Ordered list of market nodes in the path.
    pub path_nodes: Vec<String>,
    /// Edge weights (correlation strength) between consecutive nodes.
    pub edge_weights: Vec<u64>,
    /// Decay factors per hop (basis points, 10000 = no decay).
    pub decay_factors: Vec<u64>,
    /// Total propagation latency estimate in milliseconds.
    pub total_latency_ms: u64,
    /// Number of hops in the path.
    pub hop_count: u8,
    /// Timestamp of path computation.
    pub computed_at: i64,
    /// Bump seed for PDA derivation.
    pub bump: u8,
}

impl PropagationPath {
    // discriminator + signal + authority + path_nodes + edge_weights
    //   + decay_factors + total_latency_ms + hop_count + computed_at + bump
    pub const SIZE: usize = 8
        + 32
        + 32
        + (4 + MAX_PATH_HOPS * (4 + MAX_MARKET_LEN))
        + (4 + MAX_PATH_HOPS * 8)
        + (4 + MAX_PATH_HOPS * 8)
        + 8
        + 1
        + 8
        + 1;
}

/// Stores the predicted impact at a downstream market.
#[account]
#[derive(Default)]
pub struct ImpactPrediction {
    /// The propagation path this prediction is based on.
    pub propagation_path: Pubkey,
    /// Authority that generated this prediction.
    pub authority: Pubkey,
    /// Predicted impact magnitude in basis points.
    pub predicted_magnitude_bps: u64,
    /// Lower bound of confidence interval (bps).
    pub confidence_lower_bps: u64,
    /// Upper bound of confidence interval (bps).
    pub confidence_upper_bps: u64,
    /// Confidence level in basis points (e.g. 9500 = 95%).
    pub confidence_level_bps: u64,
    /// Estimated time to arrival in milliseconds.
    pub eta_ms: u64,
    /// Unix timestamp of prediction generation.
    pub predicted_at: i64,
    /// Whether the alert has been emitted.
    pub alert_emitted: bool,
    /// Target market identifier.
    pub target_market: String,
    /// Bump seed for PDA derivation.
    pub bump: u8,
}

impl ImpactPrediction {
    // discriminator + propagation_path + authority + predicted_magnitude_bps
    //   + confidence_lower_bps + confidence_upper_bps + confidence_level_bps
    //   + eta_ms + predicted_at + alert_emitted + target_market + bump
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + (4 + MAX_MARKET_LEN) + 1;
}

/// Global protocol state account.
#[account]
#[derive(Default)]
pub struct ProtocolState {
    /// Protocol admin authority.
    pub admin: Pubkey,
    /// Total signals detected.
    pub total_signals: u64,
    /// Total propagation paths computed.
    pub total_paths: u64,
    /// Total impact predictions generated.
    pub total_predictions: u64,
    /// Protocol version.
    pub version: u8,
    /// Bump seed for PDA derivation.
    pub bump: u8,
}

impl ProtocolState {
    // discriminator + admin + total_signals + total_paths
    //   + total_predictions + version + bump
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 1;
}
