<div align="center">

# 📜 Lumen Lens — Contracts

**Soroban smart contracts powering the Lumen Lens protocol.**

[![Soroban](https://img.shields.io/badge/Soroban-SDK_26-7D00FF?logo=stellar&logoColor=white)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-2021-DE3F24?logo=rust&logoColor=white)](https://www.rust-lang.org)

</div>

---

## Contracts

| Crate | Purpose |
|---|---|
| [`lumen_lens`](lumen_lens) | Core protocol contract — submits and stores analytics snapshots on-chain |
| [`analytics`](analytics) | Batched snapshot ingestion with rate limiting, diffing, and pause/unpause controls |
| [`access-control`](access-control) | Role- and permission-based access control shared across contracts |
| [`governance`](governance) | Proposal creation and vote tallying for protocol governance |
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
| `lumen_lens` | [`CDFFSZYJVQQJKI5O5WF63OQMPJCSY55BAZDIZGTD3XUFD764TSTYTDQK`](https://stellar.expert/explorer/testnet/contract/CDFFSZYJVQQJKI5O5WF63OQMPJCSY55BAZDIZGTD3XUFD764TSTYTDQK) | [tx](https://stellar.expert/explorer/testnet/tx/f949727ccc3001c245a88be119f3800c5117daddb348089ba15f158053f73676) | [tx](https://stellar.expert/explorer/testnet/tx/f8dfcdebdee8cd61545b0e5de22dcb63329f8abb70878f70205916e3aa64e822) |

`lumen_lens` is initialized and live on Testnet (SDF network).

## Linting

Workspace-wide Clippy lints deny `unwrap()`, `expect()`, and `panic!` in contract code (`[workspace.lints.clippy]` in `Cargo.toml`) — contracts must handle errors explicitly rather than aborting.
