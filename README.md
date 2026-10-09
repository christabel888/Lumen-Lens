<div align="center">

<img src="assets/logo.svg" alt="Lumen Lens logo" width="140" height="140" />

# 🌌 Lumen Lens

**We turn Stellar's ledger into a story you can actually read.** ✨

Cross-border payments move fast on Stellar — corridor health, anchor uptime, settlement latency, all happening in real time. We watch it so you don't have to squint at raw XDR.

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7D00FF?logo=stellar&logoColor=white)](https://stellar.org)
[![Rust](https://img.shields.io/badge/Backend-Rust-DE3F24?logo=rust&logoColor=white)](backend)
[![Next.js](https://img.shields.io/badge/Frontend-Next.js-000000?logo=nextdotjs&logoColor=white)](frontend)
[![Soroban](https://img.shields.io/badge/Contracts-Soroban_SDK_26-7D00FF?logo=stellar&logoColor=white)](contracts)

</div>

---

## 🛰️ What we're building

A lean, production-grade stack that turns raw Stellar network activity into payment reliability metrics that actually mean something — corridor success rates, anchor uptime, settlement latency — served up through a live dashboard backed by a Rust analytics engine and an on-chain analytics layer. No guesswork, just ledgers, decoded.

## 📦 What's in this repo

| Path | What's in it |
|---|---|
| 🧭 **[`frontend/`](frontend)** | Next.js dashboard, plus infra (k8s, Terraform, ELK), docs, and shared tooling |
| ⚙️ **[`backend/`](backend)** | Rust/Axum analytics engine — ingestion, alerting, caching, REST + WebSocket API |
| 📜 **[`contracts/`](contracts)** | Soroban smart contracts — analytics snapshots, access control, governance |
| 🖼️ **[`assets/`](assets)** | Logo and shared images |

## 🧰 Stack at a glance

- ⚙️ **Backend** — Rust, Axum, PostgreSQL/SQLite, Redis
- 🖥️ **Frontend** — Next.js 16, React 19, Tailwind 4, Recharts
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

**Frontend**

```bash
cd frontend
pnpm install
pnpm dev        # http://localhost:3000
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
