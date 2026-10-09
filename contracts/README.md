<div align="center">

# 📜 Stellar Analysis — Contracts

**Soroban smart contracts powering the Stellar Analysis protocol.**

[![Soroban](https://img.shields.io/badge/Soroban-SDK_26-7D00FF?logo=stellar&logoColor=white)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-2021-DE3F24?logo=rust&logoColor=white)](https://www.rust-lang.org)

</div>

---

## Contracts

| Crate | Purpose |
|---|---|
| [`stellar_analysis`](stellar_analysis) | Core protocol contract — submits and stores analytics snapshots on-chain |
| [`analytics`](analytics) | Batched snapshot ingestion with rate limiting, diffing, and pause/unpause controls |
| [`access-control`](access-control) | Role- and permission-based access control shared across contracts |
| [`escrow`](escrow) | Escrow service for holding and releasing funds between parties |
| [`governance`](governance) | Proposal creation and vote tallying for protocol governance |
| [`governance-voting`](governance-voting) | Voter registration and weighted voting on governance proposals |
| [`multi-sig-wallet`](multi-sig-wallet) | Multi-signature wallet with configurable owner threshold |
| [`time-locked-transactions`](time-locked-transactions) | Scheduled transfers that unlock at a future ledger time |
| [`token-swap`](token-swap) | On-chain offer creation and settlement for token swaps |
| [`upgrade`](upgrade) | Governance-gated contract upgrade proposals and approvals |
| [`benches`](benches) | Wasm size / CPU benchmarks for the contract suite |

All contracts share a Cargo workspace (`Cargo.toml`) and the same `soroban-sdk` / `soroban-token-sdk` versions.

## Prerequisites

- Rust (stable) with the `wasm32-unknown-unknown` target:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

Optimized release builds strip symbols and use `opt-level = "z"` (see the workspace `[profile.release]`) to minimize deployed Wasm size.

## Test

```bash
cargo test
```

Benchmarks live in `benches/` and use the `[profile.bench]` profile (`opt-level = 3`).

## Deploy

Requires Rust 1.84+ target `wasm32v1-none` (the `wasm32-unknown-unknown` target on
Rust 1.82+ enables Wasm features the Soroban environment doesn't yet support):

```bash
rustup target add wasm32v1-none
cargo build --target wasm32v1-none --release -p <package_name>

soroban contract deploy \
  --wasm target/wasm32v1-none/release/<contract_name>.wasm \
  --source <account> \
  --network testnet

soroban contract invoke --id <contract_id> --source <account> --network testnet \
  -- initialize --admin <account_address>
```

### Live on testnet

| Contract | Contract ID | Deploy tx | Init tx |
|---|---|---|---|
| `stellar_analysis` | [`CDFFSZYJVQQJKI5O5WF63OQMPJCSY55BAZDIZGTD3XUFD764TSTYTDQK`](https://stellar.expert/explorer/testnet/contract/CDFFSZYJVQQJKI5O5WF63OQMPJCSY55BAZDIZGTD3XUFD764TSTYTDQK) | [tx](https://stellar.expert/explorer/testnet/tx/f949727ccc3001c245a88be119f3800c5117daddb348089ba15f158053f73676) | [tx](https://stellar.expert/explorer/testnet/tx/f8dfcdebdee8cd61545b0e5de22dcb63329f8abb70878f70205916e3aa64e822) |
| `escrow` | [`CBWB5JIKJC7TJ2FBXB6AZEJ6HLOOBZCKWUN2RNTXTFFXQMRTLG2WDDNU`](https://stellar.expert/explorer/testnet/contract/CBWB5JIKJC7TJ2FBXB6AZEJ6HLOOBZCKWUN2RNTXTFFXQMRTLG2WDDNU) | [tx](https://stellar.expert/explorer/testnet/tx/d50332f9ce0aaea462129b05d3ce8566bb09e361caf28eb4b460688f1923d085) | [tx](https://stellar.expert/explorer/testnet/tx/7afcff0d5e8499560de044ebc24c93e3a2ef165a1f87a05d3cf393a460604885) |

Both are initialized and live on Testnet (SDF network). The other 8 contracts in
this workspace are not yet deployed anywhere.

## Linting

Workspace-wide Clippy lints deny `unwrap()`, `expect()`, and `panic!` in contract code (`[workspace.lints.clippy]` in `Cargo.toml`) — contracts must handle errors explicitly rather than aborting.

## Related repos

- [backend](https://github.com/Stellar-Analysis/backend) — indexes and serves the on-chain data these contracts produce
- [frontend](https://github.com/Stellar-Analysis/frontend/tree/main/frontend) — dashboard consuming this data
- [mobile](https://github.com/Stellar-Analysis/mobile) — mobile client
