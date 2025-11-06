use anchor_lang::prelude::*;

#[account]
pub struct Market {
    pub authority: Pubkey,           // Admin who can resolve
    pub market_id: String,           // Unique market ID
    pub question: String,            // Market question
    pub total_yes_pool: u64,         // Total SOL bet on YES (in lamports)
    pub total_no_pool: u64,          // Total SOL bet on NO (in lamports)
    pub status: MarketStatus,        // active, locked, resolved
    pub winning_outcome: Option<u8>, // None, Some(0=NO), Some(1=YES)
    pub resolved_at: Option<i64>,    // Timestamp
    pub created_at: i64,             // Timestamp
    pub bump: u8,                    // PDA bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MarketStatus {
    Active,
    Locked,
    Resolved,
}

#[account]
pub struct Bet {
    pub market: Pubkey,              // Market account
    pub user: Pubkey,                // Bettor's wallet
    pub outcome: u8,                 // 0 = NO, 1 = YES
    pub amount: u64,                 // Bet amount in lamports
    pub claimed: bool,               // Has payout been claimed?
    pub placed_at: i64,              // Timestamp
    pub bump: u8,                    // PDA bump
}

impl Market {
    pub const MAX_SIZE: usize = 8 +  // discriminator
        32 +                          // authority
        (4 + 50) +                    // market_id (String)
        (4 + 200) +                   // question (String)
        8 +                           // total_yes_pool
        8 +                           // total_no_pool
        (1 + 1) +                     // status (enum)
        (1 + 1) +                     // winning_outcome (Option<u8>)
        (1 + 8) +                     // resolved_at (Option<i64>)
        8 +                           // created_at
        1;                            // bump
}

impl Bet {
    pub const MAX_SIZE: usize = 8 +  // discriminator
        32 +                          // market
        32 +                          // user
        1 +                           // outcome
        8 +                           // amount
        1 +                           // claimed
        8 +                           // placed_at
        1;                            // bump
}
