use serde_json::json;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::service_client::SERVICE_CLIENT;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::service_privileges::SERVICE_PRIVILEGES;

#[tokio::test]
async fn awaits_the_signing_keys_of_a_client() {
    let fixture = ProviderFixture::serving(
        FixtureClients {
            asserting: vec![AssertingClient::awaiting_its_keys(
                SERVICE_CLIENT,
                SERVICE_PRIVILEGES,
            )],
            public: Vec::new(),
        },
        Vec::new(),
    )
    .await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 503);
    assert_eq!(answer.body["error"], "temporarily_unavailable");

    fixture.stop().await;
}
