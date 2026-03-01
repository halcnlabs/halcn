use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::HalcnError;
use crate::events::PropagationComputed;
use crate::state::{PropagationPath, SignalAccount};
use crate::utils::validate_market_name;

#[derive(Accounts)]
pub struct Propagate<'info> {
    #[account(
        init,
        payer = authority,
        space = PropagationPath::SIZE,
        seeds = [PROPAGATION_SEED, signal.key().as_ref()],
        bump,
    )]
    pub propagation_path: Account<'info, PropagationPath>,

    #[account(
        mut,
        has_one = authority,
        constraint = !signal.consumed @ HalcnError::SignalAlreadyConsumed,
    )]
    pub signal: Account<'info, SignalAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Propagate>,
    path_nodes: Vec<String>,
    edge_weights: Vec<u64>,
    decay_factors: Vec<u64>,
) -> Result<()> {
    require!(!path_nodes.is_empty(), HalcnError::EmptyPath);
    require!(path_nodes.len() <= MAX_PATH_HOPS, HalcnError::PathTooLong);
    require!(
        edge_weights.len() == path_nodes.len().saturating_sub(1),
        HalcnError::PathWeightMismatch
    );
    require!(
        decay_factors.len() == path_nodes.len().saturating_sub(1),
        HalcnError::PathDecayMismatch
    );

    for name in &path_nodes {
        require!(
            validate_market_name(name, MAX_MARKET_LEN),
            HalcnError::MarketNameTooLong
        );
    }
    for &decay in &decay_factors {
        require!(decay <= BPS_DENOMINATOR, HalcnError::DecayFactorInvalid);
        require!(decay >= MIN_DECAY_FACTOR, HalcnError::DecayFactorTooLow);
    }

    let total_latency_ms: u64 = edge_weights.iter().sum();
    let hop_count = path_nodes.len().saturating_sub(1) as u8;
    let clock = Clock::get()?;

    let prop = &mut ctx.accounts.propagation_path;
    prop.signal = ctx.accounts.signal.key();
    prop.authority = ctx.accounts.authority.key();
    prop.path_nodes = path_nodes;
    prop.edge_weights = edge_weights;
    prop.decay_factors = decay_factors;
    prop.total_latency_ms = total_latency_ms;
    prop.hop_count = hop_count;
    prop.computed_at = clock.unix_timestamp;
    prop.bump = ctx.bumps.propagation_path;

    ctx.accounts.signal.consumed = true;

    emit!(PropagationComputed {
        propagation_path: prop.key(),
        signal: ctx.accounts.signal.key(),
        authority: ctx.accounts.authority.key(),
        hop_count,
        total_latency_ms,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "Propagation path computed: hops={}, latency={}ms",
        hop_count,
        total_latency_ms
    );

    Ok(())
}
