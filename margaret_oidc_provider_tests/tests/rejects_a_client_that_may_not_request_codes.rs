use margaret_accepted_clients::authorization_code_grant::AuthorizationCodeGrant;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::portal_client::portal_client;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;

#[tokio::test]
async fn rejects_a_client_that_may_not_request_codes() {
    let mut portal = portal_client();

    portal.authorization_code = AuthorizationCodeGrant::Withheld;

    let fixture = ProviderFixture::serving(vec![portal], Vec::new()).await;
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
