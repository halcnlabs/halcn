use crate::constants::BPS_DENOMINATOR;

pub fn compute_decay_chain(factors: &[u64]) -> Option<u64> {
    let mut magnitude = BPS_DENOMINATOR;
    for &factor in factors {
        magnitude = magnitude
            .checked_mul(factor)?
            .checked_div(BPS_DENOMINATOR)?;
    }
    Some(magnitude)
}

pub fn confidence_interval_half_width(magnitude_bps: u64, confidence_bps: u64) -> Option<u64> {
    let uncertainty = BPS_DENOMINATOR.checked_sub(confidence_bps)?;
    uncertainty
        .checked_mul(magnitude_bps)?
        .checked_div(BPS_DENOMINATOR)
}

pub fn validate_market_name(name: &str, max_len: usize) -> bool {
    !name.is_empty() && name.len() <= max_len && name.is_ascii()
}

pub fn clamp(value: u64, min: u64, max: u64) -> u64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
