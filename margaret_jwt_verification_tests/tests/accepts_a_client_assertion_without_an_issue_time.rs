use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jwt_verification::client_assertion_profile::ClientAssertionProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn accepts_a_client_assertion_without_an_issue_time() {
    let trust = fixture_trust();
    let SignedClaims {
        key_set: KeySetAssembly::Assembled(key_set),
        token,
    } = SignedClaims::new(&json!({ "aud": trust.audience, "iss": trust.issuer, "exp": 1_000 }))
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_serialized_jwt::<Map<String, Value>, ClientAssertionProfile>(
            &key_set,
            &token,
            &trust.expectation(),
            NumericDate::new(950)
        ),
        JwtVerification::Verified(_)
    ));
}
