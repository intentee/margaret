use sqlx::query;
use tokio::join;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn postgres_provider_refuses_a_code_that_expires_while_its_tokens_are_prepared() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let fixture = ProviderFixture::over_state(postgres.state.clone()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let digest = TokenDigest::of(&code);
    let exchange = code_exchange(&code);
    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(r#"SELECT 1 FROM "margaret-oidc-authorization-codes" WHERE "digest" = $1 FOR UPDATE"#)
        .bind(digest.as_bytes().as_slice())
        .execute(&mut *holder)
        .await
        .expect("the lock holder locks the code");

    let (answer, ()) = join!(
        fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
        async {
            await_blocked_sessions(pool, 1).await;
            query(
                r#"UPDATE "margaret-oidc-authorization-codes" SET "expires_at" = to_timestamp(0) WHERE "digest" = $1"#,
            )
            .bind(digest.as_bytes().as_slice())
            .execute(&mut *holder)
            .await
            .expect("the lock holder expires the code");
            holder
                .commit()
                .await
                .expect("the lock holder releases the code");
        },
    );

    assert_eq!(answer.status, 400);
    assert_eq!(
        answer.body["error_description"],
        "the authorization code is not known"
    );

    fixture.stop().await;
}
