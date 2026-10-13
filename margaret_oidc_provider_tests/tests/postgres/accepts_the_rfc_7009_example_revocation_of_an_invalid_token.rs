use std::sync::Arc;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::rfc_example_client::RFC_EXAMPLE_CLIENT;
use margaret_oidc_provider_tests::rfc_example_privileges::RFC_EXAMPLE_PRIVILEGES;
use margaret_oidc_provider_tests::started_with_provider_state::started_with_provider_state;

const REQUEST_BODY: &str = include_str!("../../fixtures/rfc7009/section_2_1_request_body.txt");

#[tokio::test]
async fn accepts_the_rfc_7009_example_revocation_of_an_invalid_token() {
    let database = started_with_provider_state().await;
    let clients = FixtureClients {
        asserting: vec![AssertingClient::holding_its_keys(
            RFC_EXAMPLE_CLIENT,
            RFC_EXAMPLE_PRIVILEGES,
            Arc::clone(&database.database),
        )],
        public: Vec::new(),
    };
    let fixture = ProviderFixture::serving(database, clients, Vec::new()).await;
    let answer = fixture
        .post_encoded_form("/revoke", RFC_EXAMPLE_CLIENT.client_id, REQUEST_BODY)
        .await;

    assert_eq!(answer.status, 200);

    fixture.stop().await;
}
