use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::HalcnError;
use crate::state::{PropagationPath, SignalAccount};

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

    let total_latency_ms: u64 = edge_weights.iter().sum();

    let prop = &mut ctx.accounts.propagation_path;
    prop.signal = ctx.accounts.signal.key();
    prop.authority = ctx.accounts.authority.key();
    prop.path_nodes = path_nodes;
    prop.edge_weights = edge_weights;
    prop.decay_factors = decay_factors;
    prop.total_latency_ms = total_latency_ms;
    prop.computed_at = Clock::get()?.unix_timestamp;
    prop.bump = ctx.bumps.propagation_path;

    ctx.accounts.signal.consumed = true;

    Ok(())
}
