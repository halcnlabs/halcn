use anchor_lang::prelude::*;

#[error_code]
pub enum HalcnError {
    #[msg("Threshold out of valid range")]
    ThresholdOutOfRange,

    #[msg("Window size out of valid range")]
    WindowOutOfRange,

    #[msg("Market identifier exceeds maximum length")]
    MarketNameTooLong,

    #[msg("Unauthorized signer")]
    Unauthorized,
}
