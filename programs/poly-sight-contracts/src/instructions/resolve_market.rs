use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct ResolveMarket<'info> {
    #[account(
        mut,
        seeds = [b"market", market_id.as_bytes()],
        bump = market.bump,
        constraint = market.status == MarketStatus::Active @ PredictionMarketError::MarketNotActive,
        constraint = market.authority == authority.key() @ PredictionMarketError::Unauthorized
    )]
    pub market: Account<'info, Market>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<ResolveMarket>,
    _market_id: String,
    winning_outcome: u8,
) -> Result<()> {
    require!(winning_outcome <= 1, PredictionMarketError::InvalidOutcome);
    
    let market = &mut ctx.accounts.market;
    let clock = Clock::get()?;
    
    market.status = MarketStatus::Resolved;
    market.winning_outcome = Some(winning_outcome);
    market.resolved_at = Some(clock.unix_timestamp);
    
    msg!(
        "Market resolved: {} - Winner: {}",
        market.market_id,
        if winning_outcome == 1 { "YES" } else { "NO" }
    );
    
    Ok(())
}
