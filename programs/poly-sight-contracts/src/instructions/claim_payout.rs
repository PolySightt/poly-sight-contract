use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ClaimPayout<'info> {
    #[account(
        seeds = [b"market", market.market_id.as_bytes()],
        bump = market.bump,
        constraint = market.status == MarketStatus::Resolved @ PredictionMarketError::MarketNotResolved
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        mut,
        seeds = [
            b"bet",
            market.key().as_ref(),
            user.key().as_ref(),
            &bet.placed_at.to_le_bytes()
        ],
        bump = bet.bump,
        constraint = bet.user == user.key() @ PredictionMarketError::Unauthorized,
        constraint = !bet.claimed @ PredictionMarketError::AlreadyClaimed,
        constraint = Some(bet.outcome) == market.winning_outcome @ PredictionMarketError::NotWinner
    )]
    pub bet: Account<'info, Bet>,
    
    /// CHECK: Escrow PDA
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

pub fn handler(ctx: Context<ClaimPayout>) -> Result<()> {
    let market = &ctx.accounts.market;
    let bet = &mut ctx.accounts.bet;
    
    // Calculate payout
    let total_pool = market.total_yes_pool
        .checked_add(market.total_no_pool)
        .ok_or(PredictionMarketError::Overflow)?;
    
    let winning_pool = if bet.outcome == 1 {
        market.total_yes_pool
    } else {
        market.total_no_pool
    };
    
    require!(winning_pool > 0, PredictionMarketError::DivisionByZero);
    
    // Payout = (user_bet / winning_pool) * total_pool
    let payout = (bet.amount as u128)
        .checked_mul(total_pool as u128)
        .ok_or(PredictionMarketError::Overflow)?
        .checked_div(winning_pool as u128)
        .ok_or(PredictionMarketError::DivisionByZero)?
        as u64;
    
    // Platform fee (2%)
    let fee = payout.checked_mul(2).unwrap().checked_div(100).unwrap();
    let net_payout = payout.checked_sub(fee).unwrap();
    
    // Transfer from escrow to user
    let market_key = market.key();
    let seeds = &[
        b"escrow",
        market_key.as_ref(),
        &[ctx.bumps.escrow],
    ];
    let signer_seeds = &[&seeds[..]];
    
    **ctx.accounts.escrow.try_borrow_mut_lamports()? -= net_payout;
    **ctx.accounts.user.try_borrow_mut_lamports()? += net_payout;
    
    bet.claimed = true;
    
    msg!(
        "Payout claimed: {} SOL (fee: {} SOL)",
        net_payout as f64 / 1_000_000_000.0,
        fee as f64 / 1_000_000_000.0
    );
    
    Ok(())
}
