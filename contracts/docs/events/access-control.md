# `access-control` — Event Reference

`access-control/src/lib.rs` emits 5 real on-chain events today.

| Event | Topic(s) | Trigger | Consumer |
|---|---|---|---|
| `InitializedEvent` | `(symbol_short!("ac_init"),)` | Every `initialize` call (the function has no re-init guard, so callers must only call it once per deployment) | New Deployments panel |
| `RoleGrantedEvent` | `(symbol_short!("role_grnt"), user)` | Every successful `grant_role` call (caller must hold `Admin` or higher) | Audit trail / permissions insight |
| `RoleRevokedEvent` | `(symbol_short!("role_rvk"), user)` | Every `revoke_role` call where `user` has a roles entry in storage at all — fires even if `user` didn't hold the specific role being revoked; does **not** fire if `user` was never granted any role | Audit trail / permissions insight |
| `PermissionGrantedEvent` | `(symbol_short!("perm_grnt"), role)` | Every successful `grant_permission` call | Audit trail / permissions insight |
| upgrade | `(symbol_short!("upgrade"),)` | Every successful Wasm upgrade (SuperAdmin-only) | New Deployments panel |

## Field reference

**`InitializedEvent`**: `admin` (the address initialized with), `timestamp`, `ledger_sequence`.

**`RoleGrantedEvent` / `RoleRevokedEvent`**: `admin` (caller), `user` (target), `role` (the single role granted/revoked — a user may hold others not listed here).

**`PermissionGrantedEvent`**: `admin` (caller), `role`, `function` (a Soroban `Symbol`, not a full signature).

**upgrade**: untyped tuple `(caller: Address, new_wasm_hash: BytesN<32>)`.

See `docs/EVENTS.md` for the cross-crate summary.
