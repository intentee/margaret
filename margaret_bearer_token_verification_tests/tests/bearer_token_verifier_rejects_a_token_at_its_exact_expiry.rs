use serde_json::json;

use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_bearer_token_verification_tests::access_token_verifier_holding::access_token_verifier_holding;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen_tests::published_key_set::published_key_set;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn bearer_token_verifier_rejects_a_token_at_its_exact_expiry() {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret.current().sign_json(
        &json!({
            "aud": trust.audience.as_str(),
            "exp": 1_700_000_000,
            "iat": 1_699_999_100,
            "iss": trust.issuer.as_str(),
            "sub": "subject",
        }),
        JwtType::AccessToken,
    );
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        published_key_set(&secret)
    else {
        panic!("the published key set is accepted");
    };

    assert!(matches!(
        access_token_verifier_holding(key_set).verify::<TestClaims>(
            &RequestAuthorization::parse(Some(&format!("Bearer {token}"))),
            NumericDate::new(1_700_000_000),
        ),
        BearerTokenVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Expired { .. }))
    ));
}
