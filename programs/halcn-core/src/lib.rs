use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod halcn_core {
    use super::*;

    pub fn detect_signal(
        ctx: Context<DetectSignal>,
        source_market: String,
        threshold: u64,
        window_ms: u64,
    ) -> Result<()> {
        instructions::detect_signal::handler(ctx, source_market, threshold, window_ms)
    }
}
