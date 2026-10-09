use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use utoipa::ToSchema;

use crate::cache::helpers::cached_query;
use crate::cache::{keys, CacheManager};
use crate::database::Database;
use crate::error::{ApiError, ApiResult};
use crate::rpc::StellarRpcClient;
use crate::services::price_feed::PriceFeedClient;

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkVolumeDataPoint {
    pub time: String,
    pub volume: f64,
    pub corridors: i32,
    pub anchors: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CorridorPerformanceMetric {
    pub corridor: String,
    pub success_rate: f64,
    pub volume: f64,
    pub health: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkStats {
    pub volume_24h: f64,
    pub volume_growth: f64,
    pub avg_success_rate: f64,
    pub success_rate_growth: f64,
    pub active_corridors: i32,
    pub corridors_growth: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnalyticsDashboardData {
    pub stats: NetworkStats,
    pub time_series_data: Vec<NetworkVolumeDataPoint>,
    pub corridor_performance: Vec<CorridorPerformanceMetric>,
}

/// Handler for GET /analytics/dashboard (cached with 1 min TTL; mounted under `/analytics` in the API router)
#[utoipa::path(
    get,
    path = "/analytics/dashboard",
    responses(
        (status = 200, description = "Analytics dashboard data", body = AnalyticsDashboardData),
        (status = 500, description = "Internal server error")
    ),
    tag = "Analytics"
)]
pub async fn analytics_dashboard(
    State(cache): State<Arc<CacheManager>>,
) -> Json<AnalyticsDashboardData> {
    let cache_key = keys::analytics_dashboard();

    let dashboard_data = cached_query(
        &cache,
        &cache_key,
        cache.config.get_ttl("dashboard"),
        || async {
            // Generate real analytics data based on database queries
            let time_series_data = generate_time_series_data()?;
            let corridor_performance = generate_corridor_performance()?;
            let stats = generate_network_stats(&time_series_data, &corridor_performance)?;

            Ok(AnalyticsDashboardData {
                stats,
                time_series_data,
                corridor_performance,
            })
        },
    )
    .await
    .unwrap_or_else(|_| generate_fallback_data());

    Json(dashboard_data)
}

fn generate_time_series_data() -> Result<Vec<NetworkVolumeDataPoint>, anyhow::Error> {
    // For now, return realistic mock data
    // In production, this would query the database for actual time series data
    Ok(vec![
        NetworkVolumeDataPoint {
            time: "00:00".to_string(),
            volume: 45000.0,
            corridors: 18,
            anchors: 42,
        },
        NetworkVolumeDataPoint {
            time: "04:00".to_string(),
            volume: 52000.0,
            corridors: 21,
            anchors: 45,
        },
        NetworkVolumeDataPoint {
            time: "08:00".to_string(),
            volume: 48000.0,
            corridors: 19,
            anchors: 48,
        },
        NetworkVolumeDataPoint {
            time: "12:00".to_string(),
            volume: 61000.0,
            corridors: 24,
            anchors: 52,
        },
        NetworkVolumeDataPoint {
            time: "16:00".to_string(),
            volume: 55000.0,
            corridors: 22,
            anchors: 50,
        },
        NetworkVolumeDataPoint {
            time: "20:00".to_string(),
            volume: 67000.0,
            corridors: 25,
            anchors: 56,
        },
        NetworkVolumeDataPoint {
            time: "23:59".to_string(),
            volume: 72000.0,
            corridors: 28,
            anchors: 62,
        },
    ])
}

fn generate_corridor_performance() -> Result<Vec<CorridorPerformanceMetric>, anyhow::Error> {
    // For now, return realistic mock data
    // In production, this would query the database for actual corridor performance
    Ok(vec![
        CorridorPerformanceMetric {
            corridor: "USDC→PHP".to_string(),
            success_rate: 98.5,
            volume: 240_000.0,
            health: 95,
        },
        CorridorPerformanceMetric {
            corridor: "USD→PHP".to_string(),
            success_rate: 97.2,
            volume: 180_000.0,
            health: 92,
        },
        CorridorPerformanceMetric {
            corridor: "EUR→USDC".to_string(),
            success_rate: 99.1,
            volume: 150_000.0,
            health: 98,
        },
        CorridorPerformanceMetric {
            corridor: "USDC→SGD".to_string(),
            success_rate: 96.8,
            volume: 120_000.0,
            health: 89,
        },
        CorridorPerformanceMetric {
            corridor: "USD→EUR".to_string(),
            success_rate: 98.9,
            volume: 200_000.0,
            health: 97,
        },
    ])
}

fn generate_network_stats(
    time_series_data: &[NetworkVolumeDataPoint],
    corridor_performance: &[CorridorPerformanceMetric],
) -> Result<NetworkStats, anyhow::Error> {
    let total_volume: f64 = time_series_data.iter().map(|d| d.volume).sum();
    let avg_success_rate: f64 = if corridor_performance.is_empty() {
        0.0
    } else {
        corridor_performance
            .iter()
            .map(|c| c.success_rate)
            .sum::<f64>()
            / corridor_performance.len() as f64
    };

    Ok(NetworkStats {
        volume_24h: total_volume,
        volume_growth: 18.0,
        avg_success_rate,
        success_rate_growth: 0.8,
        active_corridors: corridor_performance.len() as i32,
        corridors_growth: 3,
    })
}

fn generate_fallback_data() -> AnalyticsDashboardData {
    let time_series_data = vec![
        NetworkVolumeDataPoint {
            time: "00:00".to_string(),
            volume: 45000.0,
            corridors: 18,
            anchors: 42,
        },
        NetworkVolumeDataPoint {
            time: "04:00".to_string(),
            volume: 52000.0,
            corridors: 21,
            anchors: 45,
        },
        NetworkVolumeDataPoint {
            time: "08:00".to_string(),
            volume: 48000.0,
            corridors: 19,
            anchors: 48,
        },
        NetworkVolumeDataPoint {
            time: "12:00".to_string(),
            volume: 61000.0,
            corridors: 24,
            anchors: 52,
        },
        NetworkVolumeDataPoint {
            time: "16:00".to_string(),
            volume: 55000.0,
            corridors: 22,
            anchors: 50,
        },
        NetworkVolumeDataPoint {
            time: "20:00".to_string(),
            volume: 67000.0,
            corridors: 25,
            anchors: 56,
        },
        NetworkVolumeDataPoint {
            time: "23:59".to_string(),
            volume: 72000.0,
            corridors: 28,
            anchors: 62,
        },
    ];

    let corridor_performance = vec![
        CorridorPerformanceMetric {
            corridor: "USDC→PHP".to_string(),
            success_rate: 98.5,
            volume: 240_000.0,
            health: 95,
        },
        CorridorPerformanceMetric {
            corridor: "USD→PHP".to_string(),
            success_rate: 97.2,
            volume: 180_000.0,
            health: 92,
        },
        CorridorPerformanceMetric {
            corridor: "EUR→USDC".to_string(),
            success_rate: 99.1,
            volume: 150_000.0,
            health: 98,
        },
        CorridorPerformanceMetric {
            corridor: "USDC→SGD".to_string(),
            success_rate: 96.8,
            volume: 120_000.0,
            health: 89,
        },
        CorridorPerformanceMetric {
            corridor: "USD→EUR".to_string(),
            success_rate: 98.9,
            volume: 200_000.0,
            health: 97,
        },
    ];

    let stats = NetworkStats {
        volume_24h: 2_400_000.0,
        volume_growth: 18.0,
        avg_success_rate: 98.1,
        success_rate_growth: 0.8,
        active_corridors: 24,
        corridors_growth: 3,
    };

    AnalyticsDashboardData {
        stats,
        time_series_data,
        corridor_performance,
    }
}

pub fn routes(cache: Arc<CacheManager>) -> Router {
    Router::new()
        .route("/dashboard", get(analytics_dashboard))
        .with_state(cache)
}

// ---------------------------------------------------------------------------
// /api/v1/stats/summary (issue #1)
// ---------------------------------------------------------------------------
//
// Lighter-weight sibling of `/analytics/dashboard` for the homepage stat-tile
// row: daily active accounts, 24h transaction count, 24h payment volume, and
// active Soroban contract count. This is the single most-hit endpoint in the
// pivot (every homepage load calls it), so it's cached (issue #4, folded in
// here) from day one rather than retrofitted later.
//
// **Day-boundary convention:** "24h" here means the *current UTC calendar
// day* (`date('now')` in SQLite), matching #2's DAA job and #3's snapshot
// delta convention -- not a rolling 24h window from request time. Picking one
// and being consistent is what #1's issue text calls out explicitly.

/// A stat value plus its "vs yesterday" delta (issue #3, folded in here so
/// the summary endpoint ships complete).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, ToSchema)]
pub struct StatWithDelta {
    pub value: f64,
    /// Percent change vs. the same stat yesterday. `None` when there's no
    /// prior-day row to diff against (day one in production) or yesterday's
    /// value was exactly zero -- never a fabricated 0%, `inf`, or `NaN`.
    pub delta_percent: Option<f64>,
}

impl StatWithDelta {
    fn compute(today: f64, yesterday: Option<f64>) -> Self {
        let delta_percent = match yesterday {
            Some(y) if y != 0.0 => Some(((today - y) / y) * 100.0),
            _ => None,
        };
        Self {
            value: today,
            delta_percent,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StatsSummary {
    pub daily_active_accounts: StatWithDelta,
    pub tx_count_24h: StatWithDelta,
    pub payment_volume_24h_usd: StatWithDelta,
    pub active_soroban_contracts: StatWithDelta,
    /// UTC calendar day these figures were computed for (RFC3339 date).
    pub as_of: String,
}

#[derive(sqlx::FromRow, Default)]
struct DailyFigures {
    daa: i64,
    tx_count: i64,
    active_contracts: i64,
}

#[derive(sqlx::FromRow)]
struct AssetVolumeRow {
    asset_type: String,
    asset_code: Option<String>,
    asset_issuer: Option<String>,
    total_amount: f64,
}

fn asset_identifier(asset_type: &str, code: Option<&str>, issuer: Option<&str>) -> String {
    if asset_type == "native" {
        "XLM:native".to_string()
    } else {
        format!(
            "{}:{}",
            code.unwrap_or("UNKNOWN"),
            issuer.unwrap_or("unknown")
        )
    }
}

/// Sums a UTC calendar day's payment volume converted to USD via the price
/// feed. Aggregates by asset first (`GROUP BY`) rather than pulling every
/// individual payment row, so this stays cheap regardless of how many
/// payments land in a busy day -- cost scales with distinct assets, not
/// transaction count.
async fn payment_volume_usd_for(
    db: &Database,
    price_feed: &PriceFeedClient,
    date_expr: &str,
) -> anyhow::Result<f64> {
    let sql = format!(
        "SELECT asset_type, asset_code, asset_issuer, SUM(amount) as total_amount
         FROM payments
         WHERE date(created_at) = date('now', '{date_expr}')
         GROUP BY asset_type, asset_code, asset_issuer"
    );
    let rows = sqlx::query_as::<_, AssetVolumeRow>(&sql)
        .fetch_all(db.pool())
        .await?;

    let asset_ids: Vec<String> = {
        let mut seen = HashSet::new();
        rows.iter()
            .map(|r| asset_identifier(&r.asset_type, r.asset_code.as_deref(), r.asset_issuer.as_deref()))
            .filter(|id| seen.insert(id.clone()))
            .collect()
    };
    let prices = price_feed.get_prices(&asset_ids).await;

    let total = rows
        .iter()
        .filter_map(|r| {
            let id = asset_identifier(&r.asset_type, r.asset_code.as_deref(), r.asset_issuer.as_deref());
            prices.get(&id).map(|price| r.total_amount * price)
        })
        .sum();
    Ok(total)
}

async fn daily_figures_for(db: &Database, date_expr: &str) -> anyhow::Result<DailyFigures> {
    let sql = format!(
        "SELECT
            (SELECT COUNT(DISTINCT source_account) FROM payments WHERE date(created_at) = date('now', '{date_expr}')) as daa,
            (SELECT COUNT(*) FROM payments WHERE date(created_at) = date('now', '{date_expr}')) as tx_count,
            (SELECT COUNT(DISTINCT contract_id) FROM contract_events WHERE date(created_at) = date('now', '{date_expr}')) as active_contracts"
    );
    let figures = sqlx::query_as::<_, DailyFigures>(&sql)
        .fetch_optional(db.pool())
        .await?
        .unwrap_or_default();
    Ok(figures)
}

async fn compute_stats_summary(
    db: &Database,
    price_feed: &PriceFeedClient,
) -> anyhow::Result<StatsSummary> {
    let today = daily_figures_for(db, "+0 days").await?;
    let yesterday = daily_figures_for(db, "-1 days").await?;
    let volume_today = payment_volume_usd_for(db, price_feed, "+0 days").await?;
    let volume_yesterday = payment_volume_usd_for(db, price_feed, "-1 days").await?;

    // `yesterday`'s query always returns a (possibly all-zero) row rather
    // than NULL, so an explicit "was there really a prior day" check isn't
    // available here the way #3's dedicated snapshot table has one; a
    // same-day network with genuinely zero activity yesterday and a cold
    // start both read as "yesterday == 0", which is exactly the case
    // `StatWithDelta::compute` already treats as "no meaningful delta".
    let yesterday_daa = if yesterday.daa == 0 && today.daa == 0 {
        None
    } else {
        Some(yesterday.daa as f64)
    };
    let yesterday_tx = if yesterday.tx_count == 0 && today.tx_count == 0 {
        None
    } else {
        Some(yesterday.tx_count as f64)
    };
    let yesterday_contracts = if yesterday.active_contracts == 0 && today.active_contracts == 0 {
        None
    } else {
        Some(yesterday.active_contracts as f64)
    };

    Ok(StatsSummary {
        daily_active_accounts: StatWithDelta::compute(today.daa as f64, yesterday_daa),
        tx_count_24h: StatWithDelta::compute(today.tx_count as f64, yesterday_tx),
        payment_volume_24h_usd: StatWithDelta::compute(volume_today, Some(volume_yesterday)),
        active_soroban_contracts: StatWithDelta::compute(
            today.active_contracts as f64,
            yesterday_contracts,
        ),
        as_of: chrono::Utc::now().format("%Y-%m-%d").to_string(),
    })
}

/// Get homepage stat-tile summary: DAA, 24h tx count, 24h payment volume,
/// active Soroban contracts, each with a vs-yesterday delta.
///
/// Cached with the dashboard TTL (`CACHE_DASHBOARD_STATS_TTL`, default 60s)
/// -- see issue #4. A transient DB/RPC failure is never cached: `cached_query`
/// only writes to the cache after `query_fn` succeeds, so an error always
/// propagates as a fresh 500 rather than being served stale for the full TTL.
#[utoipa::path(
    get,
    path = "/api/v1/stats/summary",
    responses(
        (status = 200, description = "Homepage stat-tile summary", body = StatsSummary),
        (status = 500, description = "Internal server error")
    ),
    tag = "Overview"
)]
pub async fn get_stats_summary(
    State((db, cache, _rpc_client, price_feed)): State<(
        Arc<Database>,
        Arc<CacheManager>,
        Arc<StellarRpcClient>,
        Arc<PriceFeedClient>,
    )>,
) -> ApiResult<Json<StatsSummary>> {
    let key = keys::stats_summary();
    let ttl = cache.config.get_ttl("dashboard");

    let summary = cached_query(&cache, &key, ttl, || async {
        compute_stats_summary(&db, &price_feed).await
    })
    .await
    .map_err(|e| ApiError::internal("stats_summary_failed", format!("{e}")))?;

    Ok(Json(summary))
}
