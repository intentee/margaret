use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_callback::PORTAL_CALLBACK;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::signed_in_end_user::signed_in_end_user;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn issues_a_code_to_an_implicitly_consenting_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(portal_parameters(), "max_age", "3600")),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");
    let redirection = Redirection::of_authorization(&outcome);
    let mut callback = redirection.location.clone();

    callback.set_query(None);

    assert_eq!(callback.as_str(), PORTAL_CALLBACK);
    assert_eq!(redirection.parameter("code").len(), 43);
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
