<div align="center">

<img src="assets/logo.svg" alt="Lumen Lens logo" width="140" height="140" />

# 🌌 Lumen Lens

**We turn Stellar's ledger into a story you can actually read.** ✨

Cross-border payments move fast on Stellar — corridor health, anchor uptime, settlement latency, all happening in real time. We watch it so you don't have to squint at raw XDR.

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF?logo=stellar&logoColor=white)](https://stellar.org)
[![Rust](https://img.shields.io/badge/Backend-Rust-DE3F24?logo=rust&logoColor=white)](backend)
[![Soroban](https://img.shields.io/badge/Contracts-Soroban_SDK_26-7D00FF?logo=stellar&logoColor=white)](contracts)

</div>

---

## 🛰️ What we're building

A lean, production-grade stack that turns raw Stellar network activity into payment reliability metrics that actually mean something — corridor success rates, anchor uptime, settlement latency — served up by a Rust analytics engine over REST and WebSockets, with an on-chain analytics layer on Soroban. No guesswork, just ledgers, decoded.

## 📦 What's in this repo

| Path | What's in it |
|---|---|
| ⚙️ **[`backend/`](backend)** | Rust/Axum analytics engine — ingestion, alerting, caching, REST + WebSocket API |
| 📜 **[`contracts/`](contracts)** | Soroban smart contracts — analytics snapshots, access control, governance |
| 🖼️ **[`assets/`](assets)** | Logo and shared images |

## 🧰 Stack at a glance

- ⚙️ **Backend** — Rust, Axum, PostgreSQL/SQLite, Redis
- 📜 **Contracts** — Soroban (Rust), deployed on Stellar
- 🔭 **Observability** — OpenTelemetry, Jaeger, ELK

## 🚀 Getting started

```bash
git clone https://github.com/christabel888/Lumen-Lens.git
cd Lumen-Lens
```

**Backend**

```bash
cd backend
cp .env.example .env
cargo run
```

**Contracts**

```bash
cd contracts
rustup target add wasm32v1-none
cargo test
cargo build --target wasm32v1-none --release
```

Each folder has its own README with the full details.

---

<div align="center">

*Built on [Stellar](https://stellar.org) · Soroban smart contracts · fueled by curiosity about where the money actually goes* 🌠

</div>
