-- Time-series schema for the Network Dashboard (issue #45).
--
-- One wide row per UTC calendar day, rather than narrow per-metric rows,
-- since #15-#19's endpoints each want "give me the last N days of X" and a
-- wide table lets a single row backfill/upsert cover every metric for that
-- day without a multi-table join. `date` is the UTC calendar day the metrics
-- were computed for (see #2/#3's day-boundary convention).
CREATE TABLE IF NOT EXISTS network_daily_metrics (
    date TEXT PRIMARY KEY, -- YYYY-MM-DD, UTC calendar day
    daily_active_accounts INTEGER NOT NULL DEFAULT 0,
    new_accounts INTEGER NOT NULL DEFAULT 0,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    payment_volume_usd REAL NOT NULL DEFAULT 0,
    avg_fee_stroops REAL NOT NULL DEFAULT 0,
    median_fee_stroops REAL NOT NULL DEFAULT 0,
    active_soroban_contracts INTEGER NOT NULL DEFAULT 0,
    -- distinguishes "job ran and computed a real value" from "no row yet" for
    -- #15's gap-vs-zero requirement; a present row with is_complete=0 means a
    -- job run was interrupted partway through (see #2's partial-run note).
    is_complete INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- date is already the primary key (and therefore indexed), but the explicit
-- index documents intent and survives if the PK strategy ever changes.
CREATE INDEX IF NOT EXISTS idx_network_daily_metrics_date ON network_daily_metrics(date DESC);
