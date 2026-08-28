# `stellar_insights` — Event Reference

`stellar_insights/src/lib.rs` (via `stellar_insights/src/events.rs`) is one of
the crates in this workspace that emits real on-chain events today.

| Event | Topic(s) | Trigger | Consumer |
|---|---|---|---|
| contract-initialized | `(symbol_short!("init"), CONTRACT_LIFECYCLE)`, data: `admin: Address` | Once, on the first successful `initialize` call | New Deployments panel |
| `ContractDeployedEvent` | `(symbol_short!("deployed"), CONTRACT_LIFECYCLE)` | Once, immediately after the `init` event on the first successful `initialize` call | New Deployments panel — carries `version` so deploys don't need a follow-up `get_version` call |
| `SnapshotSubmitted` | `(SNAPSHOT_SUBMITTED, SNAPSHOT_LIFECYCLE)` | Every successful `submit_snapshot` call (after auth, admin, and epoch-monotonicity checks all pass) | Soroban Dashboard snapshot feed / analytics indexer |
| contract-paused | `(symbol_short!("paused"), CONTRACT_LIFECYCLE)`, data: `caller: Address` | Every successful `pause` call | Soroban Dashboard status panel |
| contract-unpaused | `(symbol_short!("unpaused"), CONTRACT_LIFECYCLE)`, data: `caller: Address` | Every successful `unpause` call | Soroban Dashboard status panel |
| `AdminTransferredEvent` | `(symbol_short!("admin"), CONTRACT_LIFECYCLE)` | Every successful `set_admin` call | Wallet/admin activity feed |

## `ContractDeployedEvent` fields

| Field | Meaning |
|---|---|
| `admin` | The admin address the contract was initialized with |
| `version` | `CARGO_PKG_VERSION` at initialization time, same value `get_version` returns |
| `timestamp` / `ledger_sequence` | Ledger time/sequence when the deployment was recorded |

## `SnapshotSubmitted` fields

| Field | Meaning |
|---|---|
| `hash` | 32-byte SHA-256 hash of the off-chain analytics snapshot, exactly as stored under `DataKey::Snapshots` |
| `epoch` | The epoch identifier passed to `submit_snapshot`; strictly greater than the previous latest epoch |
| `timestamp` | `env.ledger().timestamp()` at the moment of submission |
| `submitter` | The authenticated caller, which must equal the stored admin address |

See `docs/EVENTS.md` for the cross-crate summary.
