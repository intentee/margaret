use sqlx::query;
use tokio::join;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn postgres_provider_refuses_a_refresh_token_revoked_while_its_tokens_are_prepared() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let fixture = ProviderFixture::over_state(postgres.state.clone()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let refresh_token = portal_tokens(&fixture, &code).await.body["refresh_token"].clone();
    let digest = TokenDigest::of(
        refresh_token
            .as_str()
            .expect("the refresh token is a string"),
    );
    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(
        r#"SELECT 1 FROM "margaret-oidc-refresh-families" WHERE "id" = (SELECT "family" FROM "margaret-oidc-refresh-tokens" WHERE "digest" = $1) FOR UPDATE"#,
    )
    .bind(digest.as_bytes().as_slice())
    .execute(&mut *holder)
    .await
    .expect("the lock holder locks the refresh family");

    let (answer, ()) = join!(refreshed_tokens(&fixture, &refresh_token, None), async {
        await_blocked_sessions(pool, 1).await;
        query(
            r#"DELETE FROM "margaret-oidc-refresh-families" WHERE "id" = (SELECT "family" FROM "margaret-oidc-refresh-tokens" WHERE "digest" = $1)"#,
        )
        .bind(digest.as_bytes().as_slice())
        .execute(&mut *holder)
        .await
        .expect("the lock holder revokes the refresh family");
        holder
            .commit()
            .await
            .expect("the lock holder releases the refresh family");
    });

    assert_eq!(answer.status, 400);
    assert_eq!(
        answer.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}
