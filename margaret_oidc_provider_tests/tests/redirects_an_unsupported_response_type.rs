use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn redirects_an_unsupported_response_type() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(
                portal_parameters(),
                "response_type",
                "token",
            )),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let redirection = Redirection::of_authorization(&outcome);

    assert_eq!(redirection.parameter("error"), "unsupported_response_type");
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
