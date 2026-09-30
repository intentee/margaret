use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::without_parameter::without_parameter;

#[tokio::test]
async fn redirects_a_request_without_a_code_challenge() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&without_parameter(portal_parameters(), "code_challenge")),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let redirection = Redirection::of_authorization(&outcome);

    assert_eq!(redirection.parameter("error"), "invalid_request");
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
