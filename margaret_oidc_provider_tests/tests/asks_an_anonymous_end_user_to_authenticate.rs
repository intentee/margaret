use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::pkce_challenge::pkce_challenge;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn asks_an_anonymous_end_user_to_authenticate() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&portal_parameters()),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::AuthenticationRequired { return_to } = outcome else {
        panic!("an anonymous end user must authenticate");
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
