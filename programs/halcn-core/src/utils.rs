use crate::constants::BPS_DENOMINATOR;

/// Compute the cumulative decay across a chain of hop factors.
pub fn compute_decay_chain(factors: &[u64]) -> Option<u64> {
    let mut magnitude = BPS_DENOMINATOR;
    for &factor in factors {
        magnitude = magnitude
            .checked_mul(factor)?
            .checked_div(BPS_DENOMINATOR)?;
    }
    Some(magnitude)
}

/// Compute the confidence interval half-width.
pub fn confidence_interval_half_width(magnitude_bps: u64, confidence_bps: u64) -> Option<u64> {
    let uncertainty = BPS_DENOMINATOR.checked_sub(confidence_bps)?;
    uncertainty
        .checked_mul(magnitude_bps)?
        .checked_div(BPS_DENOMINATOR)
}
