/// Maximum length of a market identifier string.
pub const MAX_MARKET_LEN: usize = 32;

/// Maximum number of hops in a propagation path.
pub const MAX_PATH_HOPS: usize = 8;

/// Minimum detection threshold in basis points.
pub const MIN_THRESHOLD_BPS: u64 = 10;

/// Maximum detection threshold in basis points.
pub const MAX_THRESHOLD_BPS: u64 = 5000;

/// Maximum sliding window size in milliseconds (60 seconds).
pub const MAX_WINDOW_MS: u64 = 60_000;

/// Minimum sliding window size in milliseconds (100ms).
pub const MIN_WINDOW_MS: u64 = 100;

/// Basis points denominator (100% = 10000).
pub const BPS_DENOMINATOR: u64 = 10_000;

/// Maximum confidence level in basis points.
pub const MAX_CONFIDENCE_BPS: u64 = 9999;

/// Minimum confidence level in basis points.
pub const MIN_CONFIDENCE_BPS: u64 = 5000;

/// Maximum time horizon for predictions in milliseconds (5 minutes).
pub const MAX_TIME_HORIZON_MS: u64 = 300_000;

/// Minimum decay factor per hop (bps).
pub const MIN_DECAY_FACTOR: u64 = 100;

/// Default signal detection window in milliseconds.
pub const DEFAULT_WINDOW_MS: u64 = 5_000;

/// Default confidence level for predictions (95%).
pub const DEFAULT_CONFIDENCE_BPS: u64 = 9500;

/// Seed prefix for signal account PDAs.
pub const SIGNAL_SEED: &[u8] = b"signal";

/// Seed prefix for propagation path PDAs.
pub const PROPAGATION_SEED: &[u8] = b"propagation";

/// Seed prefix for impact prediction PDAs.
pub const PREDICTION_SEED: &[u8] = b"prediction";

/// Seed prefix for protocol state PDA.
pub const PROTOCOL_SEED: &[u8] = b"protocol";

/// Current protocol version.
pub const PROTOCOL_VERSION: u8 = 1;
