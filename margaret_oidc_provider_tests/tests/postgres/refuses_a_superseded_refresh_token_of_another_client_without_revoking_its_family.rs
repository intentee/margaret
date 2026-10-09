use std::sync::Arc;

use serde_json::json;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::portal_client::PORTAL_CLIENT;
use margaret_oidc_provider_tests::portal_code_grant::PORTAL_CODE_GRANT;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_privileges::PORTAL_PRIVILEGES;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_oidc_provider_tests::service_client::SERVICE_CLIENT;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::service_privileges::SERVICE_PRIVILEGES;
use margaret_oidc_provider_tests::started_with_provider_state::started_with_provider_state;

#[tokio::test]
async fn refuses_a_superseded_refresh_token_of_another_client_without_revoking_its_family() {
    let database = started_with_provider_state().await;
    let clients = FixtureClients {
        asserting: vec![
            AssertingClient::holding_its_keys_with_code_grant(
                PORTAL_CLIENT,
                PORTAL_PRIVILEGES,
                Arc::clone(&database.database),
                PORTAL_CODE_GRANT,
                vec![PORTAL_CALLBACK.to_string()],
            ),
            AssertingClient::holding_its_keys_with_code_grant(
                SERVICE_CLIENT,
                SERVICE_PRIVILEGES,
                Arc::clone(&database.database),
                PORTAL_CODE_GRANT,
                vec![PORTAL_CALLBACK.to_string()],
            ),
        ],
        public: Vec::new(),
    };
    let fixture = ProviderFixture::serving(database, clients, Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let superseded =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let current = refreshed_tokens(&fixture, &superseded, None).await.body["refresh_token"].clone();
    let foreign = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "refresh_token", "refresh_token": superseded}),
        )
        .await;

    assert_eq!(foreign.status, 400);
    assert_eq!(foreign.body["error"], "invalid_grant");
    assert_eq!(refreshed_tokens(&fixture, &current, None).await.status, 200);

    fixture.stop().await;
}
