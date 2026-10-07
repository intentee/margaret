use sqlx::query;
use tokio::join;

use margaret_provider_state_storage::assertion_refusal::AssertionRefusal;
use margaret_provider_state_storage::assertion_spending::AssertionSpending;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage_tests::assertion_clock::AssertionClock;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::fresh_digest::fresh_digest;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_state_spends_a_client_assertion_behind_a_held_lock_once() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let clock = AssertionClock::start();
    let assertion = fresh_digest();

    query(
        r#"INSERT INTO "margaret-oidc-client-assertions" ("client_id", "digest", "expires_at") VALUES ('portal', $1, now() - interval '1 hour')"#,
    )
    .bind(assertion.as_bytes().as_slice())
    .execute(pool)
    .await
    .expect("an expired record of the assertion is stored");

    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(
        r#"SELECT 1 FROM "margaret-oidc-client-assertions" WHERE "client_id" = 'portal' AND "digest" = $1 FOR UPDATE"#,
    )
    .bind(assertion.as_bytes().as_slice())
    .execute(&mut *holder)
    .await
    .expect("the lock holder locks the expired record");

    let spend = || {
        postgres
            .state
            .spend_client_assertion("portal", assertion, clock.in_seconds(60), clock.now)
    };
    let (first, second, ()) = join!(spend(), spend(), async {
        await_blocked_sessions(pool, 2).await;
        holder
            .commit()
            .await
            .expect("the lock holder releases the record");
    });
    let spendings = [
        first.expect("the backend spends the assertion"),
        second.expect("the backend spends the assertion"),
    ];

    assert!(spendings.contains(&AssertionSpending::Spent));
    assert!(spendings.contains(&AssertionSpending::Refused(AssertionRefusal::Replayed)));
}
