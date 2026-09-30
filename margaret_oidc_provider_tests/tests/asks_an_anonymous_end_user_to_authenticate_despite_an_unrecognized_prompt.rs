use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;

use crate::authorization_request::authorization_request;
use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn asks_an_anonymous_end_user_to_authenticate_despite_an_unrecognized_prompt() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            authorization_request(&with_parameter(
                portal_parameters(),
                "prompt",
                "select_account",
            )),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::AuthenticationRequired { return_to } = outcome else {
        panic!("an unrecognized prompt keeps the interactive flow");
    };

    assert!(!return_to.query_pairs().any(|(name, _)| name == "prompt"));

    fixture.stop().await;
}
