<div align="center">

# ⚙️ Lumen Lens — Backend

**Rust analytics engine for real-time Stellar payment reliability.**

[![Rust](https://img.shields.io/badge/Rust-Axum-DE3F24?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![PostgreSQL](https://img.shields.io/badge/DB-PostgreSQL%20%2F%20SQLite-4169E1?logo=postgresql&logoColor=white)](https://www.postgresql.org)
[![OpenTelemetry](https://img.shields.io/badge/Observability-OpenTelemetry-425CC7?logo=opentelemetry&logoColor=white)](https://opentelemetry.io)

</div>

---

## What it does

Ingests Stellar network activity (via RPC/Horizon), computes corridor and anchor reliability metrics, and serves them over REST, GraphQL, and WebSockets — with caching, rate limiting, alerting, and full observability built in.

## Prerequisites

- Rust (stable)
- PostgreSQL (production) or SQLite (development, default)
- Redis (caching, rate limiting)

## Setup

1. Copy the environment template and fill in required values:

   ```bash
   cp .env.example .env
   ```

   At minimum, set `JWT_SECRET`, `ENCRYPTION_KEY`, and `SEP10_SERVER_PUBLIC_KEY` — the server refuses to start with placeholder values.

2. Run database migrations:

   ```bash
   ./scripts/migrate.sh
   ```

3. Start the server:

   ```bash
   cargo run
   ```

   The server listens on `SERVER_HOST:SERVER_PORT` (default `127.0.0.1:8080`).

### Docker

```bash
docker build -t lumen-lens-backend .
docker run --env-file .env -p 8080:8080 lumen-lens-backend
```

The container entrypoint (`entrypoint.sh`) runs pending migrations before starting the server.

## Project layout

| Path | Contents |
|---|---|
| `src/api/` | REST endpoint handlers (anchors, corridors, wallets, price feed, ...) |
| `src/rpc/` | Stellar RPC/Horizon client, rate limiting, circuit breaker |
| `src/ingestion/` | Network data ingestion pipelines |
| `src/auth/`, `auth_middleware.rs` | SEP-10 Stellar auth and JWT session handling |
| `src/cache/` | Redis-backed caching and invalidation |
| `src/jobs/` | Background jobs (corridor/anchor refresh, price feed, cache cleanup) |
| `src/observability/` | OpenTelemetry tracing, health checks |
| `src/logging/` | Structured (JSON) logging, ELK/Logstash forwarding |
| `src/webhooks/` | Outbound alert delivery |
| `migrations/` | SQL migrations (32 to date) |
| `scripts/` | Migration, backup, and smoke-test scripts |

## Testing

```bash
cargo test
```

Integration/load tests live in `tests/` and `load-tests/`.

## Observability

- **Logs** — set `LOGSTASH_ENABLED=true` to forward structured logs to an ELK stack
- **Traces** — set `OTEL_ENABLED=true` to export traces to Jaeger over OTLP

## 🗺️ Roadmap

> One line per item. Pick one, open an issue that references it, and ship it in a focused PR.

**✅ What we have done**
- **Ingestion:** RPC/Horizon ingestion with a circuit breaker, a rate-limited client, and a mock Stellar RPC (`src/rpc/`) for offline development.
- **Analytics:** corridor and anchor reliability metrics, liquidity pool analysis, fee-bump tracking, account-merge detection and price feeds (`src/services/`).
- **API:** REST under `/api/v1`, WebSocket streaming at `/ws`, backfill admin routes, OAuth, SEP-10 auth with JWT sessions, and webhooks with a dispatcher.
- **Platform:** Redis caching with ETag/HTTP caching, per-client rate limiting, payload limits, request IDs, a deprecation middleware and graceful shutdown.
- **Operations:** 36 SQL migrations, backup tooling, background jobs (scheduler, backfill, daily active accounts, asset revalidation, contract event listener), and OpenTelemetry/ELK observability.

**🐛 Bug fixes**
- **Panic paths:** replace the ~320 `.unwrap()` and ~70 `.expect()` calls in `src/` with `?` and typed `error.rs` variants, then promote the `unwrap_used`/`expect_used` lints from `warn` to `deny`.
- **Dead code:** audit the 19 `#[allow(dead_code)]` sites, then either wire each one into a route or job or delete it so the compiler can flag real breakage again.
- **Toolchain drift:** pin a `rust-toolchain.toml` and get `cargo check --all-targets` and `cargo clippy -- -D warnings` passing clean on the pinned toolchain.
- **CI:** move `.github/workflows/` to the repo root with `working-directory: backend`, and replace the deprecated `actions-rs/toolchain` and `actions/cache@v3` with `dtolnay/rust-toolchain` and `Swatinem/rust-cache`.
- **Webhook delivery:** stop discarding results with `let _ =` in `services/webhook_dispatcher.rs`. Log and record each failed delivery, retry it with backoff, and dead-letter it after N attempts.

**✨ New features**
- **Contract indexing:** index `lumen_lens`, `analytics` and `governance` events into Postgres and expose them via `/api/v1/contracts/{id}/events` with cursor pagination.
- **Alerts:** add alert rules that can be configured per corridor (success-rate drop, latency spike, anchor downtime) and deliver them over the existing webhook dispatcher.
- **Exports:** add CSV/JSON export endpoints for corridor and anchor history, streamed to keep memory use flat on large ranges.
- **Multi-network:** build per-request network selection (an `X-Network` header) on top of `multi_network.rs`, so one deployment can serve testnet and mainnet side by side.
- **Health:** add a `/health/ready` probe that checks the DB, Redis and RPC separately, for container orchestration.

**📚 Documentation**
- **OpenAPI:** generate an OpenAPI spec from the Axum routes with `utoipa` (already in `Cargo.toml`) and serve Swagger UI at `/docs`.
- **Environment variables:** document every variable in `.env.example` (purpose, default, whether it is required, example).
- **Architecture:** add an `ARCHITECTURE.md` that traces a request and an ingestion cycle end to end through the modules.
- **Runbooks:** write short runbooks for backfill, migrations, cache flush and backup restore using the scripts in `scripts/`.

**🧪 Testing**
- **Coverage:** add `cargo llvm-cov` to CI with a coverage floor, starting at the current baseline and ratcheting up each release.
- **Integration tests:** run the `tests/` suite against Postgres and Redis service containers in CI instead of SQLite only.
- **Contract-shape tests:** snapshot-test every `/api/v1` JSON response with `insta` so breaking changes fail loudly.
- **Load tests:** wire the k6 scripts in `load-tests/` into a nightly job with latency budgets per endpoint.
- **Property tests:** add `proptest` cases for pagination, cursor encoding, redaction and rate-limit window math.
