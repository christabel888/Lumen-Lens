# `stellar_insights` — Event Reference

`stellar_insights/src/lib.rs` (via `stellar_insights/src/events.rs`) is one of the
three crates in this workspace that emits real on-chain events today (the other
two are `analytics` and `access-control`).

| Event | Topic(s) | Trigger | Consumer |
|---|---|---|---|
| `SnapshotSubmitted` | `(SNAPSHOT_SUBMITTED, SNAPSHOT_LIFECYCLE)` | Every successful `submit_snapshot` call (after auth, admin, and epoch-monotonicity checks all pass) | Soroban Dashboard snapshot feed / analytics indexer |
| `AnalyticsSnapshotSubmitted` (legacy) | `(SNAPSHOT_SUBMITTED,)` | Not currently called from `lib.rs` — kept for backwards compatibility with older indexers that only registered the single-topic form | none currently; do not build new integrations against this shape |
| contract-initialized | `(symbol_short!("init"), CONTRACT_LIFECYCLE)`, data: `admin: Address` | Once, on the first successful `initialize` call | New Deployments panel |
| contract-paused | `(symbol_short!("paused"), CONTRACT_LIFECYCLE)`, data: `caller: Address` | Every successful `pause` call | Soroban Dashboard status panel |
| contract-unpaused | `(symbol_short!("unpaused"), CONTRACT_LIFECYCLE)`, data: `caller: Address` | Every successful `unpause` call | Soroban Dashboard status panel |
| `AdminTransferredEvent` | `(symbol_short!("admin"), CONTRACT_LIFECYCLE)` | Every successful `set_admin` call | Wallet/admin activity feed |

## `SnapshotSubmitted` fields

| Field | Meaning |
|---|---|
| `hash` | 32-byte SHA-256 hash of the off-chain analytics snapshot, exactly as stored under `DataKey::Snapshots` |
| `epoch` | The epoch identifier passed to `submit_snapshot`; strictly greater than the previous latest epoch |
| `timestamp` | `env.ledger().timestamp()` at the moment of submission — identical to the value returned by the call |
| `submitter` | The authenticated caller, which must equal the stored admin address |

## `AdminTransferredEvent` fields

| Field | Meaning |
|---|---|
| `old_admin` | The admin address stored before this call; never empty since `initialize` always sets one first |
| `new_admin` | The address `set_admin` was called with |
| `timestamp` / `ledger_sequence` | Ledger time/sequence when the transfer was recorded |

See `docs/EVENTS.md` for the cross-crate summary and how this fits into the
Soroban Dashboard.
