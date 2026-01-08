# 🦀 Rust EVM Auto-Sender Bot

Automated EVM transaction sender for Sepolia testnet with random recipient selection.

## ⚠️ SECURITY WARNING
- **NEVER** use mainnet private keys
- **NEVER** commit `.env` file to GitHub
- Use **TESTNET ONLY** with minimal funds

## 🚀 Quick Start

### 1. Clone Repository
```bash
https://github.com/kpn011/rust-evm-test.git
cd rust-evm-auto-sender
```
# edit env 
```bash
cp .env.example .env
nano .env
```
# Install Rust (if not installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

# Build & Run
```bash
cargo build --release
```
```bash
# Run the bot
cargo run --release
```
