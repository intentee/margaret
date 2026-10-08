use std::sync::Arc;

use serde_json::json;

use margaret_accepted_clients::registered_code_grant::RegisteredCodeGrant;
use margaret_accepted_clients::remembers_client_assertions::RemembersClientAssertions;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_accepted_clients_tests::fixture_client_assertions::FixtureClientAssertions;
use margaret_authorization_grants::stores_authorization_grants::StoresAuthorizationGrants;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::fixture_authorization_grants::FixtureAuthorizationGrants;
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

#[tokio::test]
async fn refuses_a_refresh_token_of_another_client() {
    let grants: Arc<dyn StoresAuthorizationGrants> =
        Arc::new(FixtureAuthorizationGrants::undisturbed());
    let assertions: Arc<dyn RemembersClientAssertions> =
        Arc::new(FixtureClientAssertions::default());
    let code_grant = || RegisteredCodeGrant::Granted {
        grants: Arc::clone(&grants),
        policy: PORTAL_CODE_GRANT,
        redirect_uris: vec![PORTAL_CALLBACK.to_string()],
    };
    let fixture = ProviderFixture::serving(
        FixtureClients {
            asserting: vec![
                AssertingClient::holding_its_keys(
                    PORTAL_CLIENT,
                    PORTAL_PRIVILEGES,
                    Arc::clone(&assertions),
                    code_grant(),
                )
                .await,
                AssertingClient::holding_its_keys(
                    SERVICE_CLIENT,
                    SERVICE_PRIVILEGES,
                    Arc::clone(&assertions),
                    code_grant(),
                )
                .await,
            ],
            grants: Arc::clone(&grants),
            public: Vec::new(),
        },
        Vec::new(),
    )
    .await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let foreign = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "refresh_token", "refresh_token": refresh_token}),
        )
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
