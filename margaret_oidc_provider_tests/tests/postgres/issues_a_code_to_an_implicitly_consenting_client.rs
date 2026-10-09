use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn issues_a_code_to_an_implicitly_consenting_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&with_parameter(portal_parameters(), "max_age", "3600")),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");
    let AuthorizationOutcome::Redirected(response) = outcome else {
        panic!("the authorization redirects");
    };
    let redirection = Redirection::of(&response);
    let mut callback = redirection.location.clone();

    callback.set_query(None);

    assert_eq!(callback.as_str(), PORTAL_CALLBACK);
    assert_eq!(redirection.parameter("code").len(), 43);
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
