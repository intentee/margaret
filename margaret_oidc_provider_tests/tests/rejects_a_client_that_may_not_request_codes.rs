use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_accepted_clients_tests::asserting_client::AssertingClient;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::fixture_clients::FixtureClients;
use margaret_oidc_provider_tests::portal_client::PORTAL_CLIENT;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_privileges::PORTAL_PRIVILEGES;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;

#[tokio::test]
async fn rejects_a_client_that_may_not_request_codes() {
    let fixture = ProviderFixture::serving(
        FixtureClients {
            asserting: vec![AssertingClient::holding_its_keys(
                AcceptedClient {
                    authorization_code: AuthorizationCodeGrant::Withheld,
                    ..PORTAL_CLIENT
                },
                PORTAL_PRIVILEGES,
            )],
            public: Vec::new(),
        },
        Vec::new(),
    )
    .await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&portal_parameters()),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    assert!(matches!(
        outcome,
        AuthorizationOutcome::Rejected(response) if response.status() == 400
    ));

    fixture.stop().await;
}
