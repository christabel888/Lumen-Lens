//! "Vs yesterday" delta computation for the homepage stat tiles (issue #3).
//!
//! Generalizes the `volume_growth`/`success_rate_growth`/`corridors_growth`
//! pattern already used by `NetworkStats` in `src/api/analytics_dashboard.rs`
//! to the new #1 summary fields (DAA, tx count, payment volume, active
//! Soroban contracts), backed by a dedicated `daily_stat_snapshots` table
//! (migration 031) rather than reusing `NetworkStats`' ad-hoc growth fields.

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// A single stat's snapshot pair and the delta computed from it.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StatDelta {
    pub today: f64,
    /// `None` on day one in production, when there's no prior-day snapshot
    /// to diff against — not 0.0, since 0.0 would be indistinguishable from
    /// "yesterday's real value was zero".
    pub yesterday: Option<f64>,
    /// `(today - yesterday) / yesterday`, as a percentage. `None` whenever
    /// `yesterday` is `None` *or* `0.0` — both cases are undefined/misleading
    /// as a percent change, so this is explicitly omitted rather than
    /// silently emitting `0.0`, `inf`, or `NaN`. Can be negative (e.g. fee
    /// trends dropping); formatting must not assume positive-only.
    pub percent_change: Option<f64>,
}

impl StatDelta {
    #[must_use]
    pub fn compute(today: f64, yesterday: Option<f64>) -> Self {
        let percent_change = match yesterday {
            Some(y) if y != 0.0 => Some(((today - y) / y) * 100.0),
            _ => None,
        };
        Self {
            today,
            yesterday,
            percent_change,
        }
    }
}

/// Row shape for both reading and writing `daily_stat_snapshots`.
#[derive(Debug, Clone, Copy, Default, sqlx::FromRow)]
pub struct DailyStatSnapshot {
    pub daily_active_accounts: i64,
    pub tx_count: i64,
    pub payment_volume_usd: f64,
    pub active_soroban_contracts: i64,
}

/// Deltas for every homepage stat tile field.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SummaryDeltas {
    pub daily_active_accounts: StatDelta,
    pub tx_count: StatDelta,
    pub payment_volume_usd: StatDelta,
    pub active_soroban_contracts: StatDelta,
}

/// Upserts today's snapshot row. Intended to ride on #2's DAA job (or a
/// dedicated daily snapshot job) once per UTC calendar day.
pub async fn write_daily_snapshot(
    pool: &SqlitePool,
    snapshot: DailyStatSnapshot,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r"
        INSERT INTO daily_stat_snapshots
            (date, daily_active_accounts, tx_count, payment_volume_usd, active_soroban_contracts)
        VALUES (date('now'), ?1, ?2, ?3, ?4)
        ON CONFLICT(date) DO UPDATE SET
            daily_active_accounts = excluded.daily_active_accounts,
            tx_count = excluded.tx_count,
            payment_volume_usd = excluded.payment_volume_usd,
            active_soroban_contracts = excluded.active_soroban_contracts
        ",
    )
    .bind(snapshot.daily_active_accounts)
    .bind(snapshot.tx_count)
    .bind(snapshot.payment_volume_usd)
    .bind(snapshot.active_soroban_contracts)
    .execute(pool)
    .await?;
    Ok(())
}

async fn fetch_snapshot(
    pool: &SqlitePool,
    date_expr: &str,
) -> Result<Option<DailyStatSnapshot>, sqlx::Error> {
    let sql = format!(
        "SELECT daily_active_accounts, tx_count, payment_volume_usd, active_soroban_contracts
         FROM daily_stat_snapshots WHERE date = date('now', '{date_expr}')"
    );
    sqlx::query_as::<_, DailyStatSnapshot>(&sql)
        .fetch_optional(pool)
        .await
}

/// Computes deltas for every stat tile by diffing today's snapshot against
/// yesterday's. Returns `None` yesterday values (and thus `None`
/// `percent_change`) when either snapshot is missing, per the documented
/// cold-start behavior -- this must never panic or divide by zero.
pub async fn compute_summary_deltas(pool: &SqlitePool) -> Result<SummaryDeltas, sqlx::Error> {
    let today = fetch_snapshot(pool, "+0 days").await?.unwrap_or_default();
    let yesterday = fetch_snapshot(pool, "-1 days").await?;

    let y_daa = yesterday.map(|s| s.daily_active_accounts as f64);
    let y_tx = yesterday.map(|s| s.tx_count as f64);
    let y_vol = yesterday.map(|s| s.payment_volume_usd);
    let y_contracts = yesterday.map(|s| s.active_soroban_contracts as f64);

    Ok(SummaryDeltas {
        daily_active_accounts: StatDelta::compute(today.daily_active_accounts as f64, y_daa),
        tx_count: StatDelta::compute(today.tx_count as f64, y_tx),
        payment_volume_usd: StatDelta::compute(today.payment_volume_usd, y_vol),
        active_soroban_contracts: StatDelta::compute(
            today.active_soroban_contracts as f64,
            y_contracts,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_computes_percent_change() {
        let d = StatDelta::compute(120.0, Some(100.0));
        assert_eq!(d.percent_change, Some(20.0));
    }

    #[test]
    fn delta_handles_negative_change() {
        let d = StatDelta::compute(80.0, Some(100.0));
        assert_eq!(d.percent_change, Some(-20.0));
    }

    #[test]
    fn delta_is_none_when_yesterday_missing() {
        let d = StatDelta::compute(120.0, None);
        assert_eq!(d.percent_change, None);
    }

    #[test]
    fn delta_is_none_when_yesterday_zero() {
        let d = StatDelta::compute(120.0, Some(0.0));
        assert_eq!(d.percent_change, None);
    }

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::query(
            r"
            CREATE TABLE daily_stat_snapshots (
                date TEXT PRIMARY KEY,
                daily_active_accounts INTEGER NOT NULL DEFAULT 0,
                tx_count INTEGER NOT NULL DEFAULT 0,
                payment_volume_usd REAL NOT NULL DEFAULT 0,
                active_soroban_contracts INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            ",
        )
        .execute(&pool)
        .await
        .unwrap();
        pool
    }

    /// Two-day fixture: yesterday and today both seeded, verifying the math
    /// in isolation before this is wired into the live endpoint (#3).
    #[tokio::test]
    async fn two_day_fixture_computes_expected_deltas() {
        let pool = setup_db().await;

        sqlx::query(
            "INSERT INTO daily_stat_snapshots (date, daily_active_accounts, tx_count, payment_volume_usd, active_soroban_contracts)
             VALUES (date('now', '-1 days'), 100, 500, 10000.0, 5)",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO daily_stat_snapshots (date, daily_active_accounts, tx_count, payment_volume_usd, active_soroban_contracts)
             VALUES (date('now'), 150, 750, 15000.0, 8)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let deltas = compute_summary_deltas(&pool).await.unwrap();

        assert_eq!(deltas.daily_active_accounts.today, 150.0);
        assert_eq!(deltas.daily_active_accounts.yesterday, Some(100.0));
        assert_eq!(deltas.daily_active_accounts.percent_change, Some(50.0));

        assert_eq!(deltas.tx_count.percent_change, Some(50.0));
        assert_eq!(deltas.payment_volume_usd.percent_change, Some(50.0));
        assert_eq!(deltas.active_soroban_contracts.percent_change, Some(60.0));
    }

    /// Cold start: no prior snapshots at all. Must not panic or divide by
    /// zero -- every delta should come back as `None`, today as 0.
    #[tokio::test]
    async fn cold_start_returns_none_deltas() {
        let pool = setup_db().await;

        let deltas = compute_summary_deltas(&pool).await.unwrap();

        assert_eq!(deltas.daily_active_accounts.today, 0.0);
        assert_eq!(deltas.daily_active_accounts.yesterday, None);
        assert_eq!(deltas.daily_active_accounts.percent_change, None);
    }
}
