use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn rejects_a_token_at_its_exact_expiry() {
    let trust = fixture_trust();
    let SignedClaims {
        key_set: KeySetAssembly::Assembled(key_set),
        token,
    } = SignedClaims::new(
        &json!({ "aud": trust.audience.as_str(), "iss": trust.issuer.as_str(), "exp": 1_000, "iat": 900 }),
    )
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_serialized_jwt::<Map<String, Value>, IdTokenProfile>(&key_set, &token, &JwtExpectation { audience: &trust.audience, issuer: &trust.issuer }, NumericDate::new(1_000)),
        JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::Expired { exp, now }))
            if exp == NumericDate::new(1_000) && now == NumericDate::new(1_000)
    ));
}
