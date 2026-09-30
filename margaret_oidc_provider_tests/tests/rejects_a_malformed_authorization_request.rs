use validator::ValidationErrors;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_validation::validation_result::ValidationResult;

use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn rejects_a_malformed_authorization_request() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let outcome = fixture
        .authorization
        .authorize(
            ValidationResult::Invalid(ValidationErrors::new()),
            &EndUserAuthentication::Anonymous,
        )
        .await
        .expect("the authorization reaches its state");

    let AuthorizationOutcome::Rejected(response) = outcome else {
        panic!("a malformed request is never redirected");
    };

    assert_eq!(response.status(), 400);
    assert!(response.headers().iter().any(|header| {
        header.name == "content-security-policy" && header.value == "frame-ancestors 'none'"
    }));

    fixture.stop().await;
}
