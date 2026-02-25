use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod halcn_core {
    use super::*;

    /// Detect a signal event on a source market.
    pub fn detect_signal(
        ctx: Context<DetectSignal>,
        source_market: String,
        threshold: u64,
        window_ms: u64,
    ) -> Result<()> {
        instructions::detect_signal::handler(ctx, source_market, threshold, window_ms)
    }

    /// Compute the propagation path for a detected signal.
    pub fn propagate(
        ctx: Context<Propagate>,
        path_nodes: Vec<String>,
        edge_weights: Vec<u64>,
        decay_factors: Vec<u64>,
    ) -> Result<()> {
        instructions::propagate::handler(ctx, path_nodes, edge_weights, decay_factors)
    }

    /// Generate an impact prediction for a propagation path.
    pub fn predict_impact(
        ctx: Context<PredictImpact>,
        confidence_bps: u64,
        time_horizon_ms: u64,
    ) -> Result<()> {
        instructions::predict_impact::handler(ctx, confidence_bps, time_horizon_ms)
    }
}
