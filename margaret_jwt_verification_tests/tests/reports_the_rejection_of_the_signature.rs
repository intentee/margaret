use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn reports_the_rejection_of_the_signature() {
    let trust = fixture_trust();
    let KeySetAssembly::Assembled(key_set) = VerificationKeySet::assemble(vec![
        FixtureKey::generate(Curve::P256, "published").verification_key(),
    ]) else {
        panic!("the fixture key set is accepted");
    };
    let unpublished = FixtureKey::generate(Curve::P256, "unpublished");
    let token = unpublished.token(&unpublished.header(), &json!({ "exp": 1_000, "iat": 900 }));

    assert!(matches!(
        verify_serialized_jwt::<Map<String, Value>>(
            &key_set,
            &token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer,
                token_type: TypeHeaderExpectation::Optional(JwtType::Jwt)
            },
            NumericDate::new(950)
        ),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
}
