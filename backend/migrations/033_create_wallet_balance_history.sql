-- Per-wallet balance history schema backing the Wallet Dashboard's
-- balance-history endpoint (issue #47, consumed by #26).
--
-- Snapshotting is lazy/on-demand (see #26's note) -- a row only exists here
-- once someone has actually queried that address, not for every account on
-- the network, to keep this table's growth bounded.
CREATE TABLE IF NOT EXISTS wallet_balance_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    asset_code TEXT NOT NULL, -- 'XLM' for native
    asset_issuer TEXT, -- NULL for native XLM
    balance REAL NOT NULL,
    balance_usd REAL, -- NULL when the asset has no price-feed coverage (#25)
    snapshot_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- One row per address+asset+snapshot_time in practice (snapshot_at is set by
-- the snapshot job to the same value for every asset in a given run), so this
-- composite index serves both the per-address time-series read pattern and
-- de-duplication checks.
CREATE UNIQUE INDEX IF NOT EXISTS idx_wallet_balance_snapshots_unique
    ON wallet_balance_snapshots(address, asset_code, COALESCE(asset_issuer, ''), snapshot_at);
CREATE INDEX IF NOT EXISTS idx_wallet_balance_snapshots_address_time
    ON wallet_balance_snapshots(address, snapshot_at DESC);

-- Tracks which addresses are actively being snapshotted, so the (future)
-- snapshot job only visits addresses someone has looked at, rather than
-- scanning the whole network -- see #26's "lazy start" requirement.
CREATE TABLE IF NOT EXISTS wallet_snapshot_subscriptions (
    address TEXT PRIMARY KEY,
    first_queried_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_queried_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
