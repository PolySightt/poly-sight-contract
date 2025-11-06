# PolySight Contracts

<div align="center">

![Solana](https://img.shields.io/badge/Solana-14F195?style=for-the-badge&logo=solana&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-6F4FF2?style=for-the-badge&logo=anchor&logoColor=white)

**PolySight Decentralized Prediction Market Smart Contract on Solana**

[Features](#features) • [Architecture](#architecture) • [Getting Started](#getting-started) • [Documentation](#documentation) • [Security](#security)

</div>

---

## 📖 Overview

PolySight Contracts is a fully on-chain prediction market protocol built on Solana using the Anchor framework. It enables users to create binary prediction markets, place bets with SOL, and claim proportional payouts in a completely trustless manner.

All funds are secured in Program Derived Addresses (PDAs) with automated distribution based on market outcomes, ensuring transparency and eliminating counterparty risk.

### Key Highlights

- 🔒 **Trustless Escrow**: All funds locked in PDAs until market resolution
- ⚡ **High Performance**: Built on Solana for fast, low-cost transactions
- 💰 **Fair Distribution**: Proportional payouts based on winning pool
- 🛡️ **Security First**: Comprehensive validation and overflow protection
- 📊 **Transparent**: All market data and bets stored on-chain

---

## ✨ Features

### Core Functionality

| Feature | Description |
|---------|-------------|
| **Market Creation** | Initialize binary prediction markets with custom questions |
| **Binary Betting** | Place bets on YES/NO outcomes using SOL |
| **Secure Escrow** | Funds held in PDA-based escrow accounts |
| **Market Resolution** | Authority-controlled outcome determination |
| **Automated Payouts** | Proportional distribution to winning bettors |
| **Platform Fees** | 2% fee on winning payouts for sustainability |

### Technical Features

- ✅ PDA-based account derivation for security
- ✅ Checked arithmetic operations (overflow protection)
- ✅ Constraint-based account validation
- ✅ Double-claim prevention
- ✅ Minimum bet requirements (0.01 SOL)
- ✅ Comprehensive error handling

---

## 🏗️ Architecture

### Program Structure

```
programs/poly-sight-contracts/
├── src/
│   ├── lib.rs                    # Program entry point
│   ├── state.rs                  # Account structures (Market, Bet)
│   ├── errors.rs                 # Custom error definitions
│   └── instructions/
│       ├── mod.rs                # Instruction module exports
│       ├── initialize_market.rs  # Market creation logic
│       ├── place_bet.rs          # Betting logic with escrow
│       ├── resolve_market.rs     # Market resolution
│       └── claim_payout.rs       # Payout distribution
└── Cargo.toml
```

### Instructions

#### 1. Initialize Market

Creates a new prediction market with a binary question.

```rust
pub fn initialize_market(
    ctx: Context<InitializeMarket>,
    market_id: String,      // Unique market identifier (max 50 chars)
    question: String,       // Market question (max 200 chars)
) -> Result<()>
```

**Accounts:**
- `market` - PDA derived from `["market", market_id]`
- `authority` - Market creator (signer, mutable)
- `system_program` - Solana system program

**Validation:**
- Market ID length ≤ 50 characters
- Question length ≤ 200 characters

---

#### 2. Place Bet

Place a bet on a market outcome (YES or NO).

```rust
pub fn place_bet(
    ctx: Context<PlaceBet>,
    market_id: String,      // Target market ID
    outcome: u8,            // 0 = NO, 1 = YES
    amount: u64,            // Bet amount in lamports
) -> Result<()>
```

**Accounts:**
- `market` - Target market account (mutable)
- `bet` - PDA derived from `["bet", market, user, timestamp]`
- `escrow` - PDA derived from `["escrow", market]`
- `user` - Bettor (signer, mutable)
- `system_program` - Solana system program

**Validation:**
- Market must be active
- Outcome must be 0 or 1
- Amount ≥ 0.01 SOL (10,000,000 lamports)

**Flow:**
1. Validate market status and bet parameters
2. Transfer SOL from user to escrow PDA
3. Update market pool (total_yes_pool or total_no_pool)
4. Create bet account with details

---

#### 3. Resolve Market

Resolve a market with the winning outcome (authority only).

```rust
pub fn resolve_market(
    ctx: Context<ResolveMarket>,
    market_id: String,      // Target market ID
    winning_outcome: u8,    // 0 = NO, 1 = YES
) -> Result<()>
```

**Accounts:**
- `market` - Target market account (mutable)
- `authority` - Market authority (signer, mutable)

**Validation:**
- Market must be active
- Caller must be market authority
- Winning outcome must be 0 or 1

**Flow:**
1. Verify authority
2. Update market status to Resolved
3. Set winning outcome
4. Record resolution timestamp

---

#### 4. Claim Payout

Claim proportional payout for a winning bet.

```rust
pub fn claim_payout(
    ctx: Context<ClaimPayout>,
) -> Result<()>
```

**Accounts:**
- `market` - Resolved market account
- `bet` - User's bet account (mutable)
- `escrow` - Escrow PDA (mutable)
- `user` - Bettor (signer, mutable)
- `system_program` - Solana system program

**Validation:**
- Market must be resolved
- Bet must belong to user
- Bet must not be claimed
- Bet outcome must match winning outcome

**Payout Calculation:**
```
total_pool = total_yes_pool + total_no_pool
payout = (user_bet / winning_pool) * total_pool
platform_fee = payout * 0.02
net_payout = payout - platform_fee
```

**Flow:**
1. Verify bet is winning and unclaimed
2. Calculate proportional payout
3. Deduct 2% platform fee
4. Transfer net payout from escrow to user
5. Mark bet as claimed

---

### Account Structures

#### Market Account

```rust
pub struct Market {
    pub authority: Pubkey,           // Market creator/resolver
    pub market_id: String,           // Unique identifier
    pub question: String,            // Prediction question
    pub total_yes_pool: u64,         // Total SOL bet on YES
    pub total_no_pool: u64,          // Total SOL bet on NO
    pub status: MarketStatus,        // Active | Locked | Resolved
    pub winning_outcome: Option<u8>, // None | Some(0) | Some(1)
    pub resolved_at: Option<i64>,    // Resolution timestamp
    pub created_at: i64,             // Creation timestamp
    pub bump: u8,                    // PDA bump seed
}
```

**Size:** 338 bytes

---

#### Bet Account

```rust
pub struct Bet {
    pub market: Pubkey,              // Associated market
    pub user: Pubkey,                // Bettor's wallet
    pub outcome: u8,                 // 0 = NO, 1 = YES
    pub amount: u64,                 // Bet amount in lamports
    pub claimed: bool,               // Payout claimed flag
    pub placed_at: i64,              // Bet placement timestamp
    pub bump: u8,                    // PDA bump seed
}
```

**Size:** 91 bytes

---

#### Market Status Enum

```rust
pub enum MarketStatus {
    Active,      // Accepting bets
    Locked,      // No new bets (reserved for future use)
    Resolved,    // Outcome determined, payouts claimable
}
```

---

### Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 6000 | `MarketNotActive` | Market is not accepting bets |
| 6001 | `MarketNotResolved` | Market outcome not yet determined |
| 6002 | `InvalidOutcome` | Outcome must be 0 or 1 |
| 6003 | `InvalidAmount` | Bet amount must be > 0 |
| 6004 | `BetTooSmall` | Minimum bet is 0.01 SOL |
| 6005 | `Unauthorized` | Caller lacks required permissions |
| 6006 | `AlreadyClaimed` | Payout already claimed |
| 6007 | `NotWinner` | Bet did not win |
| 6008 | `Overflow` | Arithmetic overflow detected |
| 6009 | `DivisionByZero` | Division by zero attempted |
| 6010 | `MarketIdTooLong` | Market ID exceeds 50 characters |
| 6011 | `QuestionTooLong` | Question exceeds 200 characters |

---

## 🚀 Getting Started

### Prerequisites

- **Rust**: 1.76 or higher
- **Solana CLI**: 1.18 or higher
- **Anchor CLI**: 0.32.1
- **Node.js**: 16+ (for testing)
- **Yarn**: Package manager

### Installation

```bash
# Clone repository
git clone https://github.com/yourusername/poly-sight-contracts.git
cd poly-sight-contracts

# Install dependencies
yarn install

# Build program
anchor build

# Run tests
anchor test
```

### Local Development

```bash
# Start local validator
solana-test-validator

# Deploy to localnet (in another terminal)
anchor deploy

# Run tests against localnet
anchor test --skip-local-validator
```

---

## 📚 Documentation

### Program ID

`F3pfENkXG2hhtZgcJZmuwcZsf3c6qDr2FxrBuxxvaZns`

### PDA Derivation

| Account | Seeds |
|---------|-------|
| Market | `["market", market_id]` |
| Bet | `["bet", market_pubkey, user_pubkey, timestamp]` |
| Escrow | `["escrow", market_pubkey]` |

### Example Usage

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PolySightContracts } from "../target/types/poly_sight_contracts";

// Initialize program
const program = anchor.workspace.PolySightContracts as Program<PolySightContracts>;

// Create market
const marketId = "btc-100k-2024";
const question = "Will Bitcoin reach $100k by end of 2024?";

const [marketPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("market"), Buffer.from(marketId)],
  program.programId
);

await program.methods
  .initializeMarket(marketId, question)
  .accounts({
    market: marketPda,
    authority: wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

// Place bet
const betAmount = new anchor.BN(100_000_000); // 0.1 SOL
const outcome = 1; // YES

await program.methods
  .placeBet(marketId, outcome, betAmount)
  .accounts({
    market: marketPda,
    bet: betPda,
    escrow: escrowPda,
    user: wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

---

## 🛡️ Security

### Security Features

- ✅ **PDA-based Escrow**: Funds secured in program-controlled accounts
- ✅ **Checked Arithmetic**: All math operations use checked methods
- ✅ **Constraint Validation**: Anchor constraints on all account accesses
- ✅ **Authority Checks**: Resolution restricted to market creator
- ✅ **Reentrancy Protection**: State updates before external calls
- ✅ **Double-Claim Prevention**: Claimed flag prevents multiple withdrawals

### Known Limitations

- Market resolution is centralized (requires trusted authority)
- No dispute resolution mechanism
- Platform fee is hardcoded at 2%
- No market cancellation functionality

---

## 🧪 Testing

```bash
# Run all tests
anchor test

# Run specific test file
anchor test tests/poly-sight-contracts.ts

# Run with logs
anchor test -- --features "test-bpf"
```

### Test Coverage

- ✅ Market initialization
- ✅ Bet placement (YES/NO)
- ✅ Market resolution
- ✅ Payout calculation and distribution
- ✅ Error handling (invalid outcomes, unauthorized access)
- ✅ Edge cases (zero pool, minimum bet)

---

## 📊 Gas Costs (Approximate)

| Instruction | Compute Units | Transaction Fee (Devnet) |
|-------------|---------------|--------------------------|
| Initialize Market | ~15,000 | ~0.000015 SOL |
| Place Bet | ~25,000 | ~0.000025 SOL |
| Resolve Market | ~10,000 | ~0.00001 SOL |
| Claim Payout | ~20,000 | ~0.00002 SOL |

*Note: Actual costs may vary based on network congestion and account sizes.*

---

## 🗺️ Roadmap

- [ ] Multi-outcome markets (beyond binary)
- [ ] Decentralized resolution via oracle integration
- [ ] Market cancellation with refunds
- [ ] Configurable platform fees
- [ ] Time-based market expiration
- [ ] Liquidity pool integration
- [ ] Professional security audit

---

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust naming conventions
- Add tests for new features
- Update documentation for API changes
- Run `cargo fmt` and `cargo clippy` before committing

---

## 📄 License

This project is licensed under the ISC License - see the [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- **Website**: [PolySight](https://www.polysight.bet/)
- **X**: [@Polysightdotbet](https://x.com/Polysightdotbet/)

---

## 👥 Authors

- **PolySight Team** - *Initial work*
- **chauahntuan185** - *Collaboration*

---

## 🙏 Acknowledgments

- Built with [Anchor Framework](https://www.anchor-lang.com/)
- Powered by [Solana](https://solana.com/)
- Inspired by decentralized prediction market protocols

---

<div align="center">

**Built with ❤️ for the Solana ecosystem**

[⬆ Back to Top](#polysight-contracts)

</div>
