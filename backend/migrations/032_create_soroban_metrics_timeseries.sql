-- Daily rollup schema for the Soroban Dashboard (issue #46).
--
-- This is aggregated data derived from the raw `contract_events` table
-- (migration 022), NOT a duplicate of it -- #21-#24's endpoints should read
-- from this table instead of aggregating raw events on every request.
CREATE TABLE IF NOT EXISTS soroban_daily_metrics (
    date TEXT NOT NULL, -- YYYY-MM-DD, UTC calendar day
    contract_id TEXT NOT NULL, -- rolled up per-contract so #24's top-contracts
                                -- ranking and per-contract drilldowns don't need
                                -- to re-scan contract_events; a network-wide
                                -- row uses contract_id = '__all__'
    call_count INTEGER NOT NULL DEFAULT 0,
    -- Total resource cost for the day, in stroops. NULL (not 0) means no cost
    -- data was available for this contract/day -- see #23: contract_events
    -- had no cost column until this same issue batch added one, so historical
    -- rows predating that column will legitimately have no cost to roll up.
    total_gas_stroops INTEGER,
    active_contracts INTEGER NOT NULL DEFAULT 0, -- only meaningful on the
                                                   -- '__all__' network-wide row
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (date, contract_id)
);

CREATE INDEX IF NOT EXISTS idx_soroban_daily_metrics_date ON soroban_daily_metrics(date DESC);
CREATE INDEX IF NOT EXISTS idx_soroban_daily_metrics_contract ON soroban_daily_metrics(contract_id, date DESC);
