use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::without_parameter::without_parameter;

#[tokio::test]
async fn issues_a_code_without_a_requested_scope_or_state() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&without_parameter(
                without_parameter(portal_parameters(), "scope"),
                "state",
            )),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");
    let AuthorizationOutcome::Redirected(response) = outcome else {
        panic!("the authorization redirects");
    };
    let redirection = Redirection::of(&response);

    assert_eq!(redirection.parameter("code").len(), 43);
    assert!(!redirection.parameters.contains_key("state"));

    fixture.stop().await;
}
