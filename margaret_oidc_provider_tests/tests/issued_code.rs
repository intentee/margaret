use serde_json::Value;

use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::signed_in_end_user::signed_in_end_user;

pub async fn issued_code(fixture: &ProviderFixture, parameters: &Value) -> String {
    Redirection::of_authorization(
        &fixture
            .authorization
            .authorize(
                authorization_request(parameters),
                &EndUserAuthentication::Authenticated(signed_in_end_user()),
            )
            .await
            .expect("the authorization reaches its state"),
    )
    .parameter("code")
    .to_string()
}
