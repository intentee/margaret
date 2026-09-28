use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn rejects_a_token_at_its_exact_expiry() {
    let SignedClaims {
        key_set: KeySetParsing::Accepted(key_set),
        token,
    } = SignedClaims::new(&json!({ "exp": 1_000, "iat": 900 }))
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_jwt::<Map<String, Value>>(&key_set, &token, NumericDate::new(1_000)),
        JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Expired { exp, now }))
            if exp == NumericDate::new(1_000) && now == NumericDate::new(1_000)
    ));
}
