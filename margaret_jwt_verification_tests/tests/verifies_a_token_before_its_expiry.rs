use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[test]
fn verifies_a_token_before_its_expiry() {
    let SignedClaims {
        key_set: KeySetParsing::Accepted(key_set),
        token,
    } = SignedClaims::new(&json!({ "exp": 1_000, "iat": 900 }))
    else {
        panic!("the fixture key set is accepted");
    };

    let JwtVerification::Verified(verified) =
        verify_jwt::<Map<String, Value>>(&key_set, &token, NumericDate::new(999))
    else {
        panic!("the token verifies");
    };

    assert_eq!(
        verified.registered,
        RegisteredClaims {
            exp: NumericDate::new(1_000),
            iat: NumericDate::new(900),
            nbf: None,
        }
    );
}
