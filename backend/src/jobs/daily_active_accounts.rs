use anyhow::Result;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::time::{interval, Duration as TokioDuration};
use tracing::{info, warn};

use crate::observability::job_metrics::JobMetricsCollector;

/// Configuration for the daily-active-accounts job.
#[derive(Debug, Clone)]
pub struct DaaJobConfig {
    pub enabled: bool,
    pub interval_seconds: u64,
}

impl Default for DaaJobConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("JOB_DAA_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            interval_seconds: std::env::var("JOB_DAA_INTERVAL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(900),
        }
    }
}

/// Computes Daily Active Accounts (and companion counters) for the current
/// UTC calendar day and upserts the result into `network_daily_metrics`.
///
/// **Window convention:** this job uses the *UTC calendar day* (`date('now')`
/// in SQLite), not a rolling 24h window from "now". This matches
/// `network_daily_metrics.date`'s definition (see migration 031) and is the
/// convention #1/#3 build their "24h" fields and delta calculations against
/// — pick one, and this is it. A rolling-24h number would drift against the
/// snapshot-diff math in #3.
///
/// **Definition:** "active account" = a distinct `source_account` on a
/// `payments` row created today, per this issue's guidance to reuse the
/// already-extracted payment data (`ExtractedPayment.source_account` in
/// `src/ingestion/ledger.rs`) rather than re-parsing ledgers from scratch.
pub struct DailyActiveAccountsJob {
    pool: SqlitePool,
    config: DaaJobConfig,
}

impl DailyActiveAccountsJob {
    #[must_use]
    pub const fn new(pool: SqlitePool, config: DaaJobConfig) -> Self {
        Self { pool, config }
    }

    pub async fn start(self: Arc<Self>) {
        if !self.config.enabled {
            info!("Daily active accounts job is disabled");
            return;
        }

        info!(
            "Starting daily-active-accounts job (interval: {}s)",
            self.config.interval_seconds
        );

        let mut ticker = interval(TokioDuration::from_secs(self.config.interval_seconds));

        loop {
            ticker.tick().await;

            let _metrics = JobMetricsCollector::new("daily-active-accounts");
            match self.run_once().await {
                Ok(count) => {
                    info!("Daily active accounts computed: {}", count);
                    _metrics.complete_success();
                }
                Err(e) => {
                    warn!("Daily active accounts job failed: {}", e);
                    _metrics.complete_failure(&e.to_string());
                }
            }
        }
    }

    /// Computes today's DAA figure and upserts it. The full aggregate is
    /// computed in memory *before* any write happens, so a crash mid-query
    /// simply means "no write occurred this run" rather than a partial or
    /// corrupt figure landing in `network_daily_metrics` — the same
    /// all-or-nothing guarantee `contract_event_listener.rs` gets from only
    /// advancing its cursor after a batch fully succeeds.
    pub async fn run_once(&self) -> Result<i64> {
        let daa: i64 = sqlx::query_scalar(
            r"
            SELECT COUNT(DISTINCT source_account)
            FROM payments
            WHERE date(created_at) = date('now')
            ",
        )
        .fetch_one(&self.pool)
        .await?;

        let tx_count: i64 = sqlx::query_scalar(
            r"
            SELECT COUNT(*)
            FROM payments
            WHERE date(created_at) = date('now')
            ",
        )
        .fetch_one(&self.pool)
        .await?;

        sqlx::query(
            r"
            INSERT INTO network_daily_metrics (date, daily_active_accounts, transaction_count, is_complete, updated_at)
            VALUES (date('now'), ?1, ?2, 1, CURRENT_TIMESTAMP)
            ON CONFLICT(date) DO UPDATE SET
                daily_active_accounts = excluded.daily_active_accounts,
                transaction_count = excluded.transaction_count,
                is_complete = 1,
                updated_at = CURRENT_TIMESTAMP
            ",
        )
        .bind(daa)
        .bind(tx_count)
        .execute(&self.pool)
        .await?;

        Ok(daa)
    }
}
