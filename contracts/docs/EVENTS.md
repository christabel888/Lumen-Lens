# Events Reference

This is the hub document for every on-chain event emitted by the contracts in
this workspace, and what (if anything) currently consumes them. Per-crate
detail lives in `docs/events/<crate>.md`; this file is the map.

## Summary

| Crate | Emits events? | Consumed by |
|---|---|---|
| `stellar_insights` | Yes | Soroban Dashboard (snapshot feed, New Deployments panel, status panel) |
| `analytics` | Yes | Soroban Dashboard (snapshot feed, status panel) |
| `access-control` | Yes | New Deployments panel, audit trail |
| `escrow` | Storage-only today | Top Contracts ranking (planned) |
| `governance` | Storage-only today | Governance insights (planned) |
| `governance-voting` | Storage-only today | Turnout insights (planned) |
| `multi-sig-wallet` | Storage-only today | Wallet activity feed (planned) |
| `time-locked-transactions` | Storage-only today | Status panel (planned) |
| `token-swap` | Storage-only today | Top Movers ranking (planned) |
| `upgrade` | Storage-only today | New Deployments panel (planned) |

## `stellar_insights`

See [`docs/events/stellar_insights.md`](events/stellar_insights.md).
