use serde_json::json;
use sqlx::query;
use tokio::join;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_provider_state_storage_tests::await_blocked_sessions::await_blocked_sessions;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;
use margaret_token_digest::token_digest::TokenDigest;

#[tokio::test]
async fn postgres_provider_spends_a_client_assertion_presented_concurrently_once() {
    let postgres = PostgresState::with_tables().await;
    let pool = postgres.database.pool();
    let fixture =
        ProviderFixture::over_state(FixtureClients::standard(), postgres.state.clone()).await;
    let assertion = fixture.assertion("service");
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&assertion) else {
        panic!("the assertion is a compact jws");
    };
    let jti = TokenDigest::of(
        jws_claims(&jws)["jti"]
            .as_str()
            .expect("the assertion carries a jti"),
    );

    query(
        r#"INSERT INTO "margaret-oidc-client-assertions" ("client_id", "digest", "expires_at") VALUES ('service', $1, now() - interval '1 hour')"#,
    )
    .bind(jti.as_bytes().as_slice())
    .execute(pool)
    .await
    .expect("an expired record of the assertion is stored");

    let mut holder = pool.begin().await.expect("the lock holder begins");

    query(
        r#"SELECT 1 FROM "margaret-oidc-client-assertions" WHERE "client_id" = 'service' AND "digest" = $1 FOR UPDATE"#,
    )
    .bind(jti.as_bytes().as_slice())
    .execute(&mut *holder)
    .await
    .expect("the lock holder locks the expired record");

    let credentials = ClientCredentials::Assertion(assertion.clone());
    let form = json!({"grant_type": "client_credentials"});
    let (first, second, ()) = join!(
        fixture.post_form("/token", &credentials, &form),
        fixture.post_form("/token", &credentials, &form),
        async {
            await_blocked_sessions(pool, 2).await;
            holder
                .commit()
                .await
                .expect("the lock holder releases the record");
        },
    );
    let statuses = [first.status, second.status];

    assert!(statuses.contains(&200));
    assert!(statuses.contains(&401));

    fixture.stop().await;
}
