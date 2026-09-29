use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_bearer_token_verification_tests::access_token_verifier_holding::access_token_verifier_holding;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn bearer_token_verifier_accepts_a_freshly_signed_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(secret.current());
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        published_key_set(&secret)
    else {
        panic!("the published key set is accepted");
    };

    let BearerTokenVerification::Verified(verified) = access_token_verifier_holding(key_set)
        .verify::<TestClaims>(
        &RequestAuthorization::parse(Some(&format!("Bearer {token}"))),
        NumericDate::new(1_700_000_000),
    ) else {
        panic!("the freshly signed token verifies");
    };

    assert_eq!(verified.claims, claims);
}
