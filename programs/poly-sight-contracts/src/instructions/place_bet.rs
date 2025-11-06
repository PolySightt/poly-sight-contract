use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"market", market_id.as_bytes()],
        bump = market.bump,
        constraint = market.status == MarketStatus::Active @ PredictionMarketError::MarketNotActive
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        init,
        payer = user,
        space = Bet::MAX_SIZE,
        seeds = [
            b"bet",
            market.key().as_ref(),
            user.key().as_ref(),
            &Clock::get()?.unix_timestamp.to_le_bytes()
        ],
        bump
    )]
    pub bet: Account<'info, Bet>,
    
    /// CHECK: This is the escrow PDA that holds all bets
    #[account(
        mut,
        seeds = [b"escrow", market.key().as_ref()],
        bump
    )]
    pub escrow: AccountInfo<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PlaceBet>,
    _market_id: String,
    outcome: u8,
    amount: u64,
) -> Result<()> {
    require!(outcome <= 1, PredictionMarketError::InvalidOutcome);
    require!(amount > 0, PredictionMarketError::InvalidAmount);
    require!(
        amount >= 10_000_000, // Minimum 0.01 SOL
        PredictionMarketError::BetTooSmall
    );
    
    let market = &mut ctx.accounts.market;
    let bet = &mut ctx.accounts.bet;
    let clock = Clock::get()?;
    
    // Transfer SOL from user to escrow
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
            },
        ),
        amount,
    )?;
    
    // Update market pool
    if outcome == 1 {
        market.total_yes_pool = market.total_yes_pool
            .checked_add(amount)
            .ok_or(PredictionMarketError::Overflow)?;
    } else {
        market.total_no_pool = market.total_no_pool
            .checked_add(amount)
            .ok_or(PredictionMarketError::Overflow)?;
    }
    
    // Record bet
    bet.market = market.key();
    bet.user = ctx.accounts.user.key();
    bet.outcome = outcome;
    bet.amount = amount;
    bet.claimed = false;
    bet.placed_at = clock.unix_timestamp;
    bet.bump = ctx.bumps.bet;
    
    msg!(
        "Bet placed: {} SOL on {} for market {}",
        amount as f64 / 1_000_000_000.0,
        if outcome == 1 { "YES" } else { "NO" },
        market.market_id
    );
    
    Ok(())
}
