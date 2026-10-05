use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn redirects_a_prompt_combining_none_with_login() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&with_parameter(portal_parameters(), "prompt", "none login")),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::Redirected(response) = outcome else {
        panic!("the authorization redirects");
    };
    let redirection = Redirection::of(&response);

    assert_eq!(redirection.parameter("error"), "invalid_request");
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
