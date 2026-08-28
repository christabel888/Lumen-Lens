# `governance` — Event Reference

`governance/src/lib.rs` (via `governance/src/events.rs`) emits 8 real
on-chain events today, covering the full proposal lifecycle.

| Event | Topic(s) | Trigger | Consumer |
|---|---|---|---|
| `GovernanceInitialized` | `(GOV_LFE,)` | First successful `initialize` call | New Deployments panel |
| `ProposalCreated` | `(PROP_CRT, GOV_LFE)` | Every successful `create_proposal` call (upgrade-type proposal) | Governance insights / proposal timeline |
| `ParameterProposalCreatedEvent` | `(PRM_PROP, GOV_LFE)` | Every successful `create_parameter_proposal` call | Governance insights / proposal timeline |
| `VoteCastEvent` | `(VOTE_CST, GOV_LFE)` | Every successful `vote` call (one per address per proposal — `AlreadyVoted` blocks a second) | Turnout / governance insights |
| `ProposalFinalizedEvent` | `(PROP_FIN, GOV_LFE)` | Every successful `finalize` call, after the outcome (`Passed`/`Failed`) is decided by quorum + for/against | Governance insights |
| `ProposalExecutedEvent` | `(GOV_LFE,)` | Every successful `mark_executed` call (admin-only, only from `Passed`) | Governance insights / New Deployments panel (for upgrade-type proposals) |
| `GovernanceParamChangedEvent` | `(GOV_PRM, GOV_LFE)` | Every `update_quorum` / `update_voting_period` call | Governance insights / audit trail |
| `GovernanceAdminChangedEvent` | `(GOV_PRM, GOV_LFE)` | Every `set_admin` call | Audit trail |

`cleanup_proposal` (storage-rent reclamation after a proposal is no longer
`Active`) currently emits no event — tracked separately (see the paired
"add event" issue for this crate).

## Field reference

**`ProposalCreated`**: `proposal_id`, `proposer`, `target_contract`, `voting_ends_at`.

**`ParameterProposalCreatedEvent`**: same as above plus `action_label` (`"set_admin"` or `"set_paused"` — the human-readable form of the `ParameterAction` enum).

**`VoteCastEvent`**: `proposal_id`, `voter`, `choice` (the `VoteChoice` enum as `u32`: `0`=For, `1`=Against, `2`=Abstain).

**`ProposalFinalizedEvent`**: `proposal_id`, `status` (`ProposalStatus` as `u32`), `votes_for`, `votes_against`, `total_voters` — note `votes_abstain` is *not* included here even though it's part of the underlying `VoteTally`.

**`ProposalExecutedEvent`**: `proposal_id`, `executor`, `target_contract`.

**`GovernanceParamChangedEvent`**: `param_name` (`"quorum"` or `"voting_period"`), `old_value`, `new_value`, `changed_by`, `timestamp`, `ledger_sequence`.

**`GovernanceAdminChangedEvent`**: `old_admin`, `new_admin`, `changed_by`, `timestamp`, `ledger_sequence`.

## The `Proposal` / `VoteTally` structs (underlying state)

`Proposal` lives in persistent storage under `DataKey::Proposal(id)`;
`VoteTally` under `DataKey::VoteTally(proposal_id)`, created alongside it.
See the doc comments on both structs in `governance/src/lib.rs` for the
full storage-key and lifecycle detail. Both are read via `get_proposal`/
`get_tally` — events are a real-time change feed on top of this
authoritative state, not a replacement for it.

See `docs/EVENTS.md` for the cross-crate summary.
