-- Daily snapshot table backing the homepage stat tiles' "vs yesterday" delta
-- (issue #3). One row per UTC calendar day; #1's /api/v1/stats/summary
-- handler diffs today's row against yesterday's to compute percent change.
CREATE TABLE IF NOT EXISTS daily_stat_snapshots (
    date TEXT PRIMARY KEY, -- YYYY-MM-DD, UTC calendar day
    daily_active_accounts INTEGER NOT NULL DEFAULT 0,
    tx_count INTEGER NOT NULL DEFAULT 0,
    payment_volume_usd REAL NOT NULL DEFAULT 0,
    active_soroban_contracts INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_daily_stat_snapshots_date ON daily_stat_snapshots(date DESC);
