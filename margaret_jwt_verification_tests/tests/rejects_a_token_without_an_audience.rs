use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn rejects_a_token_without_an_audience() {
    let trust = fixture_trust();
    let SignedClaims {
        key_set: KeySetParsing::Accepted(key_set),
        token,
    } = SignedClaims::new(&json!({ "iss": trust.issuer.as_str(), "exp": 1_000, "iat": 900 }))
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_jwt::<Map<String, Value>>(
            &key_set,
            &token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer,
                token_type: TypeHeaderExpectation::Optional(JwtType::Jwt)
            },
            NumericDate::new(950)
        ),
        JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Malformed { .. }))
    ));
}
