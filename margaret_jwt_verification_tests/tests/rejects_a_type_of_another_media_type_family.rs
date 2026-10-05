use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::header_type::HeaderType;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::signed_claims::SignedClaims;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn rejects_a_type_of_another_media_type_family() {
    let trust = fixture_trust();
    let SignedClaims {
        key_set: KeySetAssembly::Assembled(key_set),
        token,
    } = SignedClaims::typed(
        "text/at+jwt",
        &json!({ "aud": trust.audience.as_str(), "iss": trust.issuer.as_str(), "exp": 1_000, "iat": 900 }),
    )
    else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        verify_serialized_jwt::<Map<String, Value>, AccessTokenProfile>(&key_set, &token, &trust.expectation(), NumericDate::new(950)),
        JwtVerification::Rejected(JwtRejection::Type(TypeRejection::Mismatch {
            found: HeaderType::Unsupported(found),
            ..
        })) if found == "text/at+jwt"
    ));
}
