use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn rejects_an_unregistered_redirect_uri() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&with_parameter(
                portal_parameters(),
                "redirect_uri",
                "https://attacker.example/callback",
            )),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::Rejected(response) = outcome else {
        panic!("an unregistered callback is never redirected to");
    };

    assert_eq!(response.status(), 400);

    fixture.stop().await;
}
