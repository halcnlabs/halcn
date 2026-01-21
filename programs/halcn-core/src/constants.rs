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

/// Seed prefix for signal account PDAs.
pub const SIGNAL_SEED: &[u8] = b"signal";
