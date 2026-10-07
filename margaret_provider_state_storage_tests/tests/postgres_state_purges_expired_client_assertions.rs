use sqlx::query;
use sqlx::query_scalar;

use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage_tests::assertion_clock::AssertionClock;
use margaret_provider_state_storage_tests::fresh_digest::fresh_digest;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_state_purges_expired_client_assertions() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let clock = AssertionClock::start();

    query(
        r#"INSERT INTO "margaret-oidc-client-assertions" ("client_id", "digest", "expires_at") VALUES ('stale', $1, now() - interval '1 hour')"#,
    )
    .bind(fresh_digest().as_bytes().as_slice())
    .execute(pool)
    .await
    .expect("an expired record is stored");

    assert_eq!(
        postgres
            .state
            .spend_client_assertion("portal", fresh_digest(), clock.in_seconds(60), clock.now)
            .await
            .expect("the backend spends the assertion"),
        AssertionSpending::Spent
    );
    assert_eq!(
        query_scalar::<_, i64>(
            r#"SELECT count(*) FROM "margaret-oidc-client-assertions" WHERE "client_id" = 'stale'"#
        )
        .fetch_one(pool)
        .await
        .expect("the records are counted"),
        0
    );
}
