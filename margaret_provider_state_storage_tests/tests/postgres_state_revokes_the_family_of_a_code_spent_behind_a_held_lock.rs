use sqlx::query;
use tokio::join;

use margaret_provider_state_storage::code_spending::CodeSpending;
use margaret_provider_state_storage::presented_refresh_token::PresentedRefreshToken;
use margaret_provider_state_storage::refresh_issuance::RefreshIssuance;
use margaret_provider_state_storage::stores_provider_state::StoresProviderState;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::fixture_grant::fixture_grant;
use margaret_provider_state_storage_tests::fresh_digest::fresh_digest;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_state_revokes_the_family_of_a_code_spent_behind_a_held_lock() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let code = fresh_digest();
    let first_token = fresh_digest();
    let second_token = fresh_digest();

    postgres
        .state
        .issue_code(code, fixture_grant())
        .await
        .expect("the backend stores the code");

    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(r#"SELECT 1 FROM "margaret-oidc-authorization-codes" WHERE "digest" = $1 FOR UPDATE"#)
        .bind(code.as_bytes().as_slice())
        .execute(&mut *holder)
        .await
        .expect("the lock holder locks the code");

    let (first, second, ()) = join!(
        postgres
            .state
            .spend_code(code, RefreshIssuance::Opened(first_token)),
        postgres
            .state
            .spend_code(code, RefreshIssuance::Opened(second_token)),
        async {
            await_blocked_sessions(pool, 2).await;
            holder
                .commit()
                .await
                .expect("the lock holder releases the code");
        },
    );
    let spendings = [
        first.expect("the backend spends the code"),
        second.expect("the backend spends the code"),
    ];

    assert!(spendings.contains(&CodeSpending::Spent));
    assert!(spendings.contains(&CodeSpending::Replayed));
    for token in [first_token, second_token] {
        assert_eq!(
            postgres
                .state
                .present_refresh_token(token)
                .await
                .expect("the backend looks up the token"),
            PresentedRefreshToken::Unknown
        );
    }
}
