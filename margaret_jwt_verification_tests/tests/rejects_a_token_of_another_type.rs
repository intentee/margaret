use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn rejects_a_token_of_another_type() {
    let SignedClaims {
        key_set: KeySetParsing::Accepted(key_set),
        token,
    } = SignedClaims::typed("at+jwt", &json!({ "exp": 1_000, "iat": 900 }))
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_jwt::<Map<String, Value>>(
            &key_set,
            &token,
            TypeHeaderExpectation::Optional(JwtType::Jwt),
            NumericDate::new(950)
        ),
        JwtVerification::Rejected(JwtRejection::Type(TypeRejection::Mismatch {
            expected: JwtType::Jwt,
            found: HeaderType::Supported(JwtType::AccessToken),
        }))
    ));
}
