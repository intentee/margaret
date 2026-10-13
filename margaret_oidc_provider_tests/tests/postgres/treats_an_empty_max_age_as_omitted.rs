use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider_tests::pkce_challenge::pkce_challenge;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn treats_an_empty_max_age_as_omitted() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&with_parameter(portal_parameters(), "max_age", "")),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::AuthenticationRequired { return_to } = outcome else {
        panic!("an empty maximum authentication age is no maximum");
    };

    assert_eq!(
        return_to.as_str(),
        format!(
            "https://localhost/authorize?client_id=portal&code_challenge={}&code_challenge_method=S256&nonce=n-0S6_WzA2Mj&redirect_uri=https%3A%2F%2Fportal.localhost%2Fcallback&response_type=code&scope=openid+profile&state=af0ifjsldkj",
            pkce_challenge()
        )
    );

    fixture.stop().await;
}
