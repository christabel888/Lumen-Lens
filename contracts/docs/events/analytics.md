# `analytics` — Event Reference

`analytics/src/lib.rs` is the most event-heavy contract in the workspace (24
`env.events().publish(...)` call sites). This table summarizes each one; see
the inline `//` comments directly above each `publish` call in
`analytics/src/lib.rs` for the authoritative, code-adjacent description.

| Topic | Trigger | Payload | Consumer |
|---|---|---|---|
| `("error", caller)` | Any validation/authorization failure that calls `emit_error_event` (e.g. rate limit exceeded) | `ErrorEvent` | On-chain error observability/alerting, not the happy-path dashboard feed |
| `("init", "admin")` | First successful `initialize` call | `admin: Address` (legacy, untyped) | New Deployments panel |
| `("cfg_upd", admin)` | Every successful `update_config` call | `ConfigUpdatedEvent` (old + new config) | Admin/config audit trail |
| `("snapshot", caller)` | Every successful `submit_snapshot`, per-item in `batch_submit_snapshots`, and `submit_snapshot_with_ttl` | `SnapshotSubmittedEvent` | Soroban Dashboard snapshot feed |
| `("batch", caller)` | Every `batch_submit_snapshots` call, after all per-item events | `u32` (batch size) | Snapshot feed batch summary |
| `("cleanup", admin)` | Every `cleanup_expired_snapshots` call | `u32` (count removed) | Maintenance/ops visibility |
| `("admin", new_admin)` | Every `set_admin` (both `AdminTransferEvent` and `AdminChangedEvent` fire) and `set_admin_by_governance` (`AdminChangedEvent`) | `AdminTransferEvent`, `AdminChangedEvent` | Admin/wallet activity feed |
| `("pause", caller)` | `pause`, and `set_paused_by_governance(true)` | `PauseEvent` | Status panel |
| `("unpause", caller)` | `unpause`, and `set_paused_by_governance(false)` | `UnpauseEvent` | Status panel |
| `("emergency", admin)` | `emergency_withdraw` (admin-only, only while paused) | `(token, amount, recipient)` untyped tuple | Ops/security alerting |
| `("upgrade",)` | Successful Wasm upgrade | `(admin, new_wasm_hash)` untyped tuple | New Deployments panel |
| `("gov", governance)` | `set_governance` | `GovernanceChangedEvent` | Governance insights |
| `("propose", proposer)` | `propose_admin_change` | `(action_id, new_admin, executable_at)` untyped tuple | Timelock/governance insights |
| `("tl_exec", executor)` | `execute_timelock_action` | `TimelockActionExecutedEvent` | Timelock/governance insights |
| `("tl_cncl", admin)` | `cancel_timelock_action` | `TimelockActionCancelledEvent` | Timelock/governance insights |
| `("prune", caller)` | `prune_old_snapshots` | `SnapshotsPrunedEvent` | Maintenance/ops visibility |
| `("multisig", "init")` | `initialize_multisig` | `MultiSigInitializedEvent` | Wallet dashboard |

Notes:
- Several payloads are untyped tuples rather than named `#[contracttype]`
  structs (`emergency_withdraw`, `upgrade`, `propose_admin_change`,
  `submit_snapshot_compact`). Consumers need to know the field order; see the
  inline comment at each call site.
- `submit_snapshot_with_ttl`'s event always reports `previous_epoch: 0`
  rather than the actual prior epoch — a known inconsistency versus
  `submit_snapshot`'s event, left as-is per this issue's docs-only scope.

See `docs/EVENTS.md` for the cross-crate summary.
