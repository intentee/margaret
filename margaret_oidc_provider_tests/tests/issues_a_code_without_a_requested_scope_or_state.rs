use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::signed_in_end_user::signed_in_end_user;
use crate::without_parameter::without_parameter;

#[tokio::test]
async fn issues_a_code_without_a_requested_scope_or_state() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&without_parameter(
                without_parameter(portal_parameters(), "scope"),
                "state",
            )),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");
    let redirection = Redirection::of_authorization(&outcome);

    assert_eq!(redirection.parameter("code").len(), 43);
    assert!(!redirection.parameters.contains_key("state"));

    fixture.stop().await;
}
