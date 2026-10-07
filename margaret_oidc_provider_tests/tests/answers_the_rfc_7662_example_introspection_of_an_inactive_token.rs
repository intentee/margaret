use serde_json::Value;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::rfc_example_client::RFC_EXAMPLE_CLIENT;
use margaret_oidc_provider_tests::rfc_example_privileges::RFC_EXAMPLE_PRIVILEGES;

const INACTIVE_RESPONSE: &str =
    include_str!("../fixtures/rfc7662/section_2_2_inactive_response.json");
const REQUEST_BODY: &str = include_str!("../fixtures/rfc7662/section_2_1_request_body.txt");

#[tokio::test]
async fn answers_the_rfc_7662_example_introspection_of_an_inactive_token() {
    let fixture = ProviderFixture::serving(
        FixtureClients {
            asserting: vec![AssertingClient::holding_its_keys(
                RFC_EXAMPLE_CLIENT,
                RFC_EXAMPLE_PRIVILEGES,
            )],
            public: Vec::new(),
        },
        Vec::new(),
    )
    .await;
    let answer = fixture
        .post_encoded_form("/introspect", RFC_EXAMPLE_CLIENT.client_id, REQUEST_BODY)
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(
        answer.body,
        serde_json::from_str::<Value>(INACTIVE_RESPONSE).expect("the example response is json")
    );

    fixture.stop().await;
}
