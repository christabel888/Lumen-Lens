# `analytics` — Event Reference

`analytics/src/lib.rs` is the most event-heavy contract in the workspace
(25 `env.events().publish(...)` call sites as of this change). This table
summarizes each one; see the inline `//` comments directly above each
`publish` call in `analytics/src/lib.rs` for the authoritative,
code-adjacent description.

| Topic | Trigger | Payload | Consumer |
|---|---|---|---|
| `("error", caller)` | Any validation/authorization failure that calls `emit_error_event` (e.g. rate limit exceeded) | `ErrorEvent` | On-chain error observability/alerting, not the happy-path dashboard feed |
| `("init", "admin")` | First successful `initialize` call | `admin: Address` (legacy, untyped) | New Deployments panel |
| `("cfg_upd", admin)` | Every successful `update_config` call | `ConfigUpdatedEvent` (old + new config) | Admin/config audit trail |
| `("snapshot", caller)` | Every successful `submit_snapshot`, per-item in `batch_submit_snapshots`, and `submit_snapshot_with_ttl` | `SnapshotSubmittedEvent` | Soroban Dashboard snapshot feed |
| `("batch", caller)` | Every `batch_submit_snapshots` call, after all per-item events | `u32` (batch size) | Snapshot feed batch summary |
| `("cleanup", admin)` | Every `cleanup_expired_snapshots` call | `u32` (count removed) | Maintenance/ops visibility |
| `("admin", new_admin)` | Every `set_admin` (both `AdminTransferEvent` and `AdminChangedEvent` fire) and `set_admin_by_governance` (`AdminChangedEvent`) | `AdminTransferEvent`, `AdminChangedEvent` | Admin/wallet activity feed |
| `("pause", caller)` | `pause`, and `set_paused_by_governance(true)` | `PauseEvent` | Legacy pause-only feed |
| `("unpause", caller)` | `unpause`, and `set_paused_by_governance(false)` | `UnpauseEvent` | Legacy unpause-only feed |
| `("status", caller)` | Every `pause` / `unpause` / `set_paused_by_governance` call, published alongside the topic above | `ContractStatusEvent { paused, changed_by, reason, timestamp, ledger_sequence }` | **Status panel** — single topic to subscribe to regardless of pause direction or caller path (admin vs. governance) |
| `("emergency", admin)` | `emergency_withdraw` (admin-only, only while paused) | `(token, amount, recipient)` untyped tuple | Ops/security alerting |
| `("upgrade",)` | Successful Wasm upgrade | `(admin, new_wasm_hash)` untyped tuple | New Deployments panel |
| `("gov", governance)` | `set_governance` | `GovernanceChangedEvent` | Governance insights |
| `("propose", proposer)` | `propose_admin_change` | `(action_id, new_admin, executable_at)` untyped tuple | Timelock/governance insights |
| `("tl_exec", executor)` | `execute_timelock_action` | `TimelockActionExecutedEvent` | Timelock/governance insights |
| `("tl_cncl", admin)` | `cancel_timelock_action` | `TimelockActionCancelledEvent` | Timelock/governance insights |
| `("prune", caller)` | `prune_old_snapshots` | `SnapshotsPrunedEvent` | Maintenance/ops visibility |
| `("multisig", "init")` | `initialize_multisig` | `MultiSigInitializedEvent` | Wallet dashboard |

## Why `ContractStatusEvent` was added

Before this change, the dashboard's status panel would have needed to
subscribe to both the `"pause"` and `"unpause"` topics and merge them client
side to know the contract's current paused state — and it would have missed
governance-triggered changes unless it also accounted for
`set_paused_by_governance` publishing the same two topics. `ContractStatusEvent`
is published on a single `"status"` topic on every state change (admin- or
governance-triggered), carrying the resulting `paused: bool` directly. It is
additive: `PauseEvent`/`UnpauseEvent` are still published unchanged for any
existing consumers.

Notes:
- Several payloads are untyped tuples rather than named `#[contracttype]`
  structs (`emergency_withdraw`, `upgrade`, `propose_admin_change`,
  `submit_snapshot_compact`). Consumers need to know the field order; see the
  inline comment at each call site.
- `submit_snapshot_with_ttl`'s event always reports `previous_epoch: 0`
  rather than the actual prior epoch — a known inconsistency versus
  `submit_snapshot`'s event.

See `docs/EVENTS.md` for the cross-crate summary.
