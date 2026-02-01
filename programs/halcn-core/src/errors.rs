use anchor_lang::prelude::*;

#[error_code]
pub enum HalcnError {
    #[msg("Threshold out of valid range")]
    ThresholdOutOfRange,

    #[msg("Window size out of valid range")]
    WindowOutOfRange,

    #[msg("Path exceeds maximum hop count")]
    PathTooLong,

    #[msg("Path nodes and edge weights length mismatch")]
    PathWeightMismatch,

    #[msg("Path nodes and decay factors length mismatch")]
    PathDecayMismatch,

    #[msg("Signal has already been consumed")]
    SignalAlreadyConsumed,

    #[msg("Market identifier exceeds maximum length")]
    MarketNameTooLong,

    #[msg("Empty propagation path")]
    EmptyPath,

    #[msg("Decay factor exceeds basis points denominator")]
    DecayFactorInvalid,

    #[msg("Unauthorized signer")]
    Unauthorized,
}
