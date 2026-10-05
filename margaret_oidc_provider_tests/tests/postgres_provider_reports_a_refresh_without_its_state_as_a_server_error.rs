use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_provider_state_storage_tests::postgres_state::PostgresState;

#[tokio::test]
async fn postgres_provider_reports_a_refresh_without_its_state_as_a_server_error() {
    let postgres = PostgresState::with_tables().await;
    let fixture = ProviderFixture::over_state(postgres.state.clone()).await;

    postgres.dropped().await;

    assert_eq!(
        refreshed_tokens(&fixture, &json!("tGzv3JOkF0XG5Qx2TlKWIA"), None)
            .await
            .status,
        500
    );

    fixture.stop().await;
}
