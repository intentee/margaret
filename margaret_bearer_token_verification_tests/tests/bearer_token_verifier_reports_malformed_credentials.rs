use serde_json::Value;

use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_bearer_token_verification_tests::unready_verifier::unready_verifier;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn bearer_token_verifier_reports_malformed_credentials() {
    assert!(matches!(
        unready_verifier().verify::<Value>(
            &RequestAuthorization::parse(Some("Bearer ab=c")),
            NumericDate::new(1_700_000_000),
        ),
        BearerTokenVerification::MalformedAuthorization
    ));
}
