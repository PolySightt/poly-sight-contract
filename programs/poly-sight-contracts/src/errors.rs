use anchor_lang::prelude::*;

#[error_code]
pub enum PredictionMarketError {
    #[msg("Market is not active")]
    MarketNotActive,
    
    #[msg("Market is not resolved yet")]
    MarketNotResolved,
    
    #[msg("Invalid outcome (must be 0 or 1)")]
    InvalidOutcome,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Bet amount too small (minimum 0.01 SOL)")]
    BetTooSmall,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Payout already claimed")]
    AlreadyClaimed,
    
    #[msg("Not a winner")]
    NotWinner,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Division by zero")]
    DivisionByZero,
    
    #[msg("Market ID too long (max 50 characters)")]
    MarketIdTooLong,
    
    #[msg("Question too long (max 200 characters)")]
    QuestionTooLong,
}
