# Events Reference

This is the hub document for every on-chain event emitted by the contracts in
this workspace, and what (if anything) currently consumes them. Per-crate
detail lives in `docs/events/<crate>.md`; this file is the map.

## Summary

| Crate | Emits events? | Consumed by |
|---|---|---|
| `lumen_lens` | Yes | Soroban Dashboard (snapshot feed, New Deployments panel, status panel) |
| `analytics` | Yes | Soroban Dashboard (snapshot feed, status panel, admin/governance audit trail) |
| `access-control` | Yes | New Deployments panel, audit trail |
| `governance` | Yes | Governance insights, New Deployments panel, audit trail |

## `lumen_lens`

See [`docs/events/lumen_lens.md`](events/lumen_lens.md).

## `analytics`

See [`docs/events/analytics.md`](events/analytics.md).

## `access-control`

See [`docs/events/access-control.md`](events/access-control.md).

## `governance`

See [`docs/events/governance.md`](events/governance.md).
