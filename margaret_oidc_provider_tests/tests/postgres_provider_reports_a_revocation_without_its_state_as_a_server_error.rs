use serde_json::json;

use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_provider_reports_a_revocation_without_its_state_as_a_server_error() {
    let postgres = PostgresState::with_tables().await;
    let fixture =
        ProviderFixture::over_state(FixtureClients::standard(), postgres.state.clone()).await;

    postgres.dropped().await;

    assert_eq!(
        fixture
            .post_form("/revoke", &PORTAL_CREDENTIALS, &json!({"token": "any"}))
            .await
            .status,
        500
    );

    fixture.stop().await;
}
