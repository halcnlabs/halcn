use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::HalcnError;
use crate::state::{ImpactPrediction, PropagationPath};
use crate::utils::{compute_decay_chain, confidence_interval_half_width};

#[derive(Accounts)]
pub struct PredictImpact<'info> {
    #[account(
        init,
        payer = authority,
        space = ImpactPrediction::SIZE,
        seeds = [PREDICTION_SEED, propagation_path.key().as_ref()],
        bump,
    )]
    pub prediction: Account<'info, ImpactPrediction>,

    #[account(has_one = authority)]
    pub propagation_path: Account<'info, PropagationPath>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PredictImpact>,
    confidence_bps: u64,
    time_horizon_ms: u64,
) -> Result<()> {
    require!(
        confidence_bps >= MIN_CONFIDENCE_BPS && confidence_bps <= MAX_CONFIDENCE_BPS,
        HalcnError::ConfidenceOutOfRange
    );
    require!(
        time_horizon_ms <= MAX_TIME_HORIZON_MS,
        HalcnError::TimeHorizonTooLarge
    );

    let prop = &ctx.accounts.propagation_path;

    let magnitude_bps =
        compute_decay_chain(&prop.decay_factors).ok_or(error!(HalcnError::MathOverflow))?;

    let half_width = confidence_interval_half_width(magnitude_bps, confidence_bps)
        .ok_or(error!(HalcnError::MathOverflow))?;

    let lower = magnitude_bps.saturating_sub(half_width);
    let upper = magnitude_bps.saturating_add(half_width);

    let eta_ms = prop
        .total_latency_ms
        .checked_mul(time_horizon_ms)
        .ok_or(error!(HalcnError::MathOverflow))?
        .checked_div(MAX_TIME_HORIZON_MS)
        .unwrap_or(prop.total_latency_ms);

    let clock = Clock::get()?;
    let target_market = prop.path_nodes.last().cloned().unwrap_or_default();

    let prediction = &mut ctx.accounts.prediction;
    prediction.propagation_path = prop.key();
    prediction.authority = ctx.accounts.authority.key();
    prediction.predicted_magnitude_bps = magnitude_bps;
    prediction.confidence_lower_bps = lower;
    prediction.confidence_upper_bps = upper;
    prediction.confidence_level_bps = confidence_bps;
    prediction.eta_ms = eta_ms;
    prediction.predicted_at = clock.unix_timestamp;
    prediction.alert_emitted = false;
    prediction.target_market = target_market;
    prediction.bump = ctx.bumps.prediction;

    msg!(
        "Impact predicted: magnitude={}bps, eta={}ms",
        magnitude_bps,
        eta_ms
    );

    Ok(())
}
