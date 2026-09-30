use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn redirects_a_plain_code_challenge() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(
                portal_parameters(),
                "code_challenge_method",
                "plain",
            )),
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
