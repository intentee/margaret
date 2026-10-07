use serde_json::json;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::portal_client::PORTAL_CLIENT;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::spa_client::SPA_CLIENT;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_provider_reports_a_refresh_without_its_state_as_a_server_error() {
    let postgres = PostgresState::with_tables().await;
    let fixture = ProviderFixture::over_state(
        FixtureClients {
            asserting: Vec::new(),
            public: vec![AcceptedClient {
                authorization_code: PORTAL_CLIENT.authorization_code,
                ..SPA_CLIENT
            }],
        },
        postgres.state.clone(),
    )
    .await;

    postgres.dropped().await;

    assert_eq!(
        fixture
            .post_form(
                "/token",
                &ClientCredentials::Absent,
                &json!({
                    "client_id": "spa",
                    "grant_type": "refresh_token",
                    "refresh_token": "tGzv3JOkF0XG5Qx2TlKWIA",
                    "resource": "artifacts",
                }),
            )
            .await
            .status,
        500
    );

    fixture.stop().await;
}
