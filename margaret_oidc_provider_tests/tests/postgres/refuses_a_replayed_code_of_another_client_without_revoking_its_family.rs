use std::sync::Arc;

use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
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
async fn refuses_a_replayed_code_of_another_client_without_revoking_its_family() {
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
    let code = issued_code(&redirect);
    let refresh_token = portal_tokens(&fixture, &code).await.body["refresh_token"].clone();
    let foreign = fixture
        .post_form("/token", &SERVICE_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(foreign.status, 400);
    assert_eq!(foreign.body["error"], "invalid_grant");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
