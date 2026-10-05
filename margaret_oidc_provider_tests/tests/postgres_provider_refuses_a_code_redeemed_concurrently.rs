use sqlx::query;
use tokio::join;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn postgres_provider_refuses_a_code_redeemed_concurrently() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let fixture = ProviderFixture::over_state(postgres.state.clone()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let exchange = code_exchange(&code);
    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(r#"SELECT 1 FROM "margaret-oidc-authorization-codes" WHERE "digest" = $1 FOR UPDATE"#)
        .bind(TokenDigest::of(&code).as_bytes().as_slice())
        .execute(&mut *holder)
        .await
        .expect("the lock holder locks the code");

    let (first, second, ()) = join!(
        fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
        fixture.post_form("/token", &PORTAL_CREDENTIALS, &exchange),
        async {
            await_blocked_sessions(pool, 2).await;
            holder
                .commit()
                .await
                .expect("the lock holder releases the code");
        },
    );
    let answers = [first, second];
    let redeemed = answers
        .iter()
        .find(|answer| answer.status == 200)
        .expect("one exchange redeems the code");
    let replayed = answers
        .iter()
        .find(|answer| answer.status == 400)
        .expect("the other exchange is refused");

    assert_eq!(
        replayed.body["error_description"],
        "the authorization code was already redeemed, so its tokens are revoked"
    );
    assert_eq!(
        refreshed_tokens(&fixture, &redeemed.body["refresh_token"], None)
            .await
            .status,
        400
    );

    fixture.stop().await;
}
