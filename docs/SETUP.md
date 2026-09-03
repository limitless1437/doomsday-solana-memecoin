# Development Setup

## Prerequisites
- Rust 1.70+
- Solana CLI 1.18+
- Node.js 16+ (for frontend)

## Installation

```bash
# Clone the repository
git clone https://github.com/limitless1437/doomsday-solana-memecoin.git
cd doomsday-solana-memecoin

# Install dependencies
cargo build

# Run tests
cargo test
```

## Local Testing

```bash
# Start local validator
solana-test-validator

# Deploy contract
solana program deploy target/deploy/doomsday_token.so
```

## Environment Variables
Create a `.env` file:
```
SOLANA_RPC_URL=https://api.devnet.solana.com
SOLANA_KEYPAIR=~/.config/solana/id.json
```
