use std::sync::Arc;

use serde_json::Value;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_oidc_client::oidc_token_verification::OidcTokenVerification;
use margaret_oidc_client::oidc_token_verifier::OidcTokenVerifier;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;

#[test]
fn oidc_token_verifier_challenges_malformed_credentials_as_an_invalid_request() {
    let verifier = OidcTokenVerifier::new(
        Arc::new(localhost_trust()),
        VerificationKeySetHolder::default(),
    );

    assert!(matches!(
        verifier.verify_authorization::<Value>(&RequestAuthorization::parse(Some("Bearer ab=c"))),
        Ok(OidcTokenVerification::Rejected(rejection))
            if rejection.challenge() == BearerChallenge::InvalidRequest
    ));
}
