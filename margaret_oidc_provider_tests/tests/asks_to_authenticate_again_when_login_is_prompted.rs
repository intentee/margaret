use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::signed_in_end_user::signed_in_end_user;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn asks_to_authenticate_again_when_login_is_prompted() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(portal_parameters(), "prompt", "login")),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::AuthenticationRequired { return_to } = outcome else {
        panic!("a login prompt forces the end user to authenticate again");
    };

    assert!(!return_to.query_pairs().any(|(name, _)| name == "prompt"));

    fixture.stop().await;
}
