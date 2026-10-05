use sqlx::PgPool;
use sqlx::query_scalar;
use tokio::task::yield_now;

/// # Panics
///
/// Panics when the sessions of the database cannot be inspected.
pub async fn await_blocked_sessions(pool: &PgPool, sessions: i64) {
    while query_scalar::<_, i64>(
        "SELECT count(*) FROM pg_stat_activity WHERE datname = current_database() AND cardinality(pg_blocking_pids(pid)) > 0",
    )
    .fetch_one(pool)
    .await
    .expect("the sessions of the database are inspected")
        < sessions
    {
        yield_now().await;
    }
}
