use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[test]
fn verifies_a_token_before_its_expiry() {
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

    let JwtVerification::Verified(verified) =
        verify_serialized_jwt::<Map<String, Value>, IdTokenProfile>(
            &key_set,
            &token,
            &trust.expectation(),
            NumericDate::new(999),
        )
    else {
        panic!("the token verifies");
    };

    assert_eq!(
        verified.registered,
        RegisteredClaims {
            aud: AudienceClaim::Single(trust.audience.as_str().to_string()),
            exp: NumericDate::new(1_000),
            iat: NumericDate::new(900),
            iss: trust.issuer.as_str().to_string(),
            nbf: None,
        }
    );
}
