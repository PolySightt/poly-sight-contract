use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("F3pfENkXG2hhtZgcJZmuwcZsf3c6qDr2FxrBuxxvaZns");

#[program]
pub mod poly_sight_contracts {
    use super::*;

    /// Initialize a new prediction market
    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        market_id: String,
        question: String,
    ) -> Result<()> {
        instructions::initialize_market::handler(ctx, market_id, question)
    }

    /// Place a bet on a market outcome
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        market_id: String,
        outcome: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::place_bet::handler(ctx, market_id, outcome, amount)
    }

    /// Resolve a market with winning outcome
    pub fn resolve_market(
        ctx: Context<ResolveMarket>,
        market_id: String,
        winning_outcome: u8,
    ) -> Result<()> {
        instructions::resolve_market::handler(ctx, market_id, winning_outcome)
    }

    /// Claim payout for winning bet
    pub fn claim_payout(ctx: Context<ClaimPayout>) -> Result<()> {
        instructions::claim_payout::handler(ctx)
    }
}
