use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::HalcnError;
use crate::state::SignalAccount;

#[derive(Accounts)]
#[instruction(source_market: String)]
pub struct DetectSignal<'info> {
    #[account(
        init,
        payer = authority,
        space = SignalAccount::SIZE,
        seeds = [SIGNAL_SEED, authority.key().as_ref(), source_market.as_bytes()],
        bump,
    )]
    pub signal: Account<'info, SignalAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DetectSignal>,
    source_market: String,
    threshold: u64,
    window_ms: u64,
) -> Result<()> {
    require!(source_market.len() <= MAX_MARKET_LEN, HalcnError::MarketNameTooLong);
    require!(
        threshold >= MIN_THRESHOLD_BPS && threshold <= MAX_THRESHOLD_BPS,
        HalcnError::ThresholdOutOfRange
    );
    require!(
        window_ms >= MIN_WINDOW_MS && window_ms <= MAX_WINDOW_MS,
        HalcnError::WindowOutOfRange
    );

    let signal = &mut ctx.accounts.signal;
    signal.authority = ctx.accounts.authority.key();
    signal.source_market = source_market;
    signal.threshold = threshold;
    signal.window_ms = window_ms;
    signal.detected_at = Clock::get()?.unix_timestamp;
    signal.consumed = false;
    signal.bump = ctx.bumps.signal;

    msg!("Signal detected: market={}, threshold={}", signal.source_market, threshold);

    Ok(())
}
