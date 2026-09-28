use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn keeps_registered_claims_out_of_the_application_claims() {
    let SignedClaims {
        key_set: KeySetParsing::Accepted(key_set),
        token,
    } = SignedClaims::new(&json!({ "exp": 1_000, "iat": 900, "nbf": 900, "sub": "subject" }))
    else {
        panic!("the fixture key set is accepted");
    };

    let JwtVerification::Verified(verified) =
        verify_jwt::<Map<String, Value>>(&key_set, &token, NumericDate::new(950))
    else {
        panic!("the token verifies");
    };

    assert_eq!(Value::Object(verified.claims), json!({ "sub": "subject" }));
}
