use serde_json::Value;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_request::ConsentRequest;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::provider_fixture::ProviderFixture;
use crate::signed_in_end_user::signed_in_end_user;

pub async fn requested_consent(fixture: &ProviderFixture, parameters: &Value) -> ConsentRequest {
    match fixture
        .authorization
        .authorize(
            authorization_request(parameters),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await
        .expect("the authorization reaches its state")
    {
        AuthorizationOutcome::ConsentRequired(consent) => consent,
        AuthorizationOutcome::AuthenticationRequired { .. }
        | AuthorizationOutcome::Redirected(_)
        | AuthorizationOutcome::Rejected(_) => panic!("the end user is asked for consent"),
    }
}
