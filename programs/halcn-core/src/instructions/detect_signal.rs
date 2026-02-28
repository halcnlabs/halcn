use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::HalcnError;
use crate::events::SignalDetected;
use crate::state::SignalAccount;
use crate::utils::validate_market_name;

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
    require!(
        validate_market_name(&source_market, MAX_MARKET_LEN),
        HalcnError::MarketNameTooLong
    );
    require!(
        threshold >= MIN_THRESHOLD_BPS && threshold <= MAX_THRESHOLD_BPS,
        HalcnError::ThresholdOutOfRange
    );
    require!(
        window_ms >= MIN_WINDOW_MS && window_ms <= MAX_WINDOW_MS,
        HalcnError::WindowOutOfRange
    );

    let clock = Clock::get()?;
    let signal = &mut ctx.accounts.signal;
    signal.authority = ctx.accounts.authority.key();
    signal.source_market = source_market.clone();
    signal.threshold = threshold;
    signal.window_ms = window_ms;
    signal.detected_at = clock.unix_timestamp;
    signal.consumed = false;
    signal.signal_index = 0;
    signal.bump = ctx.bumps.signal;

    emit!(SignalDetected {
        signal: signal.key(),
        authority: signal.authority,
        source_market,
        threshold,
        window_ms,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "Signal detected: market={}, threshold={}",
        signal.source_market,
        threshold
    );

    Ok(())
}
