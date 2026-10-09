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

## 🗺️ Roadmap

> One line per item. Pick one, open an issue that references it, and ship it in a focused PR.

**✅ What we have done**
- **`lumen_lens`:** snapshot submission and retrieval by epoch, latest-snapshot lookup, admin rotation, pause/unpause and an admin-gated `upgrade`. Deployed and initialized on testnet.
- **`analytics`:** batched snapshot ingestion with rate limiting, diffing and pause controls, fuzzed with `bolero`.
- **`access-control`:** role and permission grants, revokes and checks, with a `SuperAdmin` bootstrap and upgrade support.
- **`governance`:** standard and parameter proposals, voting, finalization, quorum and voting-period updates, and proposal cleanup.
- **Shared:** a Criterion benchmark suite in `benches/`, workspace Clippy lints that deny `unwrap`/`expect`/`panic`, and per-crate event docs in `docs/events/`.

**🐛 Bug fixes**
- **`access-control` re-init (security):** add a re-initialization guard to `initialize`. Today it can be called again to overwrite storage and grant a new `SuperAdmin`.
- **Return types:** make `access-control::initialize` return `Result<(), Error>` like the other contracts, so failures surface as typed errors instead of host traps.
- **Upgrade auth:** make the `upgrade` entrypoints take an explicit `caller` everywhere, matching `access-control`, so the signer and the audit event can't drift apart.
- **Storage TTL:** check that every persistent key extends its TTL on write and read, so proposals and snapshots can't expire mid-lifecycle.
- **`upgrade` events:** emit a versioned event schema on `upgrade` that includes the old and new WASM hashes, for indexers.

**✨ New features**
- **Shared roles:** have `lumen_lens`, `analytics` and `governance` gate admin calls through `access-control` roles instead of each storing its own admin.
- **Execution:** let governance call `upgrade` and the parameter setters on target contracts directly when a proposal passes, instead of the manual `mark_executed`.
- **Vote delegation:** add delegation and snapshot-based voting power to `governance` to resist flash-vote attacks.
- **Snapshot proofs:** store a Merkle root per snapshot epoch so the backend can prove individual corridor metrics on-chain.
- **Deployments:** deploy and initialize `analytics`, `access-control` and `governance` on testnet and record their IDs in the table above.

**📚 Documentation**
- **Rustdoc:** document every public entrypoint (arguments, auth required, errors, events emitted) and publish `cargo doc` output.
- **Deploy scripts:** add a scripted deploy and init for each crate with the exact `stellar contract` CLI invocations and expected outputs.
- **Error codes:** add an error-code table per crate mapping each `Error` discriminant to its meaning and how to resolve it.
- **Upgrades:** write an upgrade and migration guide covering storage layout compatibility between versions.

**🧪 Testing**
- **Fuzzing:** add `bolero`/`proptest` fuzz targets to `governance` (vote and finalize ordering) and to `access-control` (grant/revoke sequences).
- **Cross-contract tests:** add integration tests that register several contracts in one `Env` and drive governance-to-target flows.
- **Auth tests:** assert `require_auth` on every privileged entrypoint with `env.auths()`, including negative cases with the wrong signer.
- **Budgets:** turn the benchmarks into CI gates that fail on CPU/memory instruction regressions and WASM size growth.
- **Coverage:** run `cargo llvm-cov` in CI for the contracts workspace with a coverage floor per crate.
