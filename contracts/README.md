<div align="center">

# 📜 Stellar Insights — Contracts

**Soroban smart contracts powering the Stellar Insights protocol.**

[![Soroban](https://img.shields.io/badge/Soroban-SDK_26-7D00FF?logo=stellar&logoColor=white)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-2021-DE3F24?logo=rust&logoColor=white)](https://www.rust-lang.org)

</div>

---

## Contracts

| Crate | Purpose |
|---|---|
| [`stellar_insights`](stellar_insights) | Core protocol contract — submits and stores analytics snapshots on-chain |
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

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/<contract_name>.wasm \
  --source <account> \
  --network testnet
```

## Linting

Workspace-wide Clippy lints deny `unwrap()`, `expect()`, and `panic!` in contract code (`[workspace.lints.clippy]` in `Cargo.toml`) — contracts must handle errors explicitly rather than aborting.

## Related repos

- [backend](https://github.com/Stellar-Insightss/backend) — indexes and serves the on-chain data these contracts produce
- [frontend](https://github.com/Stellar-Insightss/Stellar-inights/tree/main/frontend) — dashboard consuming this data
- [mobile](https://github.com/Stellar-Insightss/mobile) — mobile client
