use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authosxrity,
        space = Market::MAX_SIZE,
        seeds = [b"market", market_id.as_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Programzz<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeMarket>,
    market_id: String,
    question: String,
) -> Result<()> {
    require!(market_id.len() <= 50, PredictionMarketError::MarketIdTooLong);
    require!(question.len() <= 200, PredictionMarketError::QuestionTooLong);
    
    let market = &mut ctx.accounts.market;
    let clock = Clock::get()?;
    
    market.authority = ctx.accounts.authority.key();
    market.market_id = market_id.clone();
    market.question = question;
    market.total_yes_pool = 0;
    market.total_no_pool = 0;
    market.status = MarketStatus::Active;
    market.winning_outcome = None;
    market.resolved_at = None;
    market.created_at = clock.unix_timestamp;
    market.bump = ctx.bumps.market;
    
    msg!("Market initialized: {}", market_id);
    
    Ok(())
}
