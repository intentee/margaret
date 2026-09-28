use std::sync::Arc;

use serde_json::Value;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_oidc_client::oidc_token_verification::OidcTokenVerification;
use margaret_oidc_client::oidc_token_verifier::OidcTokenVerifier;
use margaret_oidc_client::presented_bearer::PresentedBearer;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;

#[test]
fn oidc_token_verifier_is_unavailable_before_the_first_poll() {
    let verifier = OidcTokenVerifier::new(
        Arc::new(localhost_trust()),
        VerificationKeySetHolder::default(),
    );

    let authorization = RequestAuthorization::parse(Some("Bearer abc"));
    let presented =
        PresentedBearer::read(&authorization).expect("the system clock reads as a numeric date");

    assert!(matches!(
        verifier.verify::<Value>(&presented),
        OidcTokenVerification::Unavailable
    ));
}
