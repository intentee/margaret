use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::pkce_verifier::PKCE_VERIFIER;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::spa_callback::SPA_CALLBACK;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_provider_reports_a_code_exchange_without_its_state_as_a_server_error() {
    let postgres = PostgresState::with_tables().await;
    let fixture =
        ProviderFixture::over_state(FixtureClients::standard(), postgres.state.clone()).await;

    postgres.dropped().await;

    assert_eq!(
        fixture
            .post_form(
                "/token",
                &ClientCredentials::Absent,
                &json!({
                    "client_id": "spa",
                    "code": "SplxlOBeZQQYbYS6WxSbIA",
                    "code_verifier": PKCE_VERIFIER,
                    "grant_type": "authorization_code",
                    "redirect_uri": SPA_CALLBACK,
                    "resource": "artifacts",
                }),
            )
            .await
            .status,
        500
    );

    fixture.stop().await;
}
