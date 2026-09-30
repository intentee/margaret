use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn rejects_an_authorization_request_of_an_unknown_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(
                portal_parameters(),
                "client_id",
                "stranger",
            )),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::Rejected(response) = outcome else {
        panic!("an unknown client is never redirected");
    };

    assert_eq!(response.status(), 400);

    fixture.stop().await;
}
