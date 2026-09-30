use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_parameters::spa_parameters;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn redirects_a_prompting_client_asked_for_no_interaction() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(spa_parameters(), "prompt", "none")),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state");

    assert_eq!(
        Redirection::of_authorization(&outcome).parameter("error"),
        "consent_required"
    );

    fixture.stop().await;
}
