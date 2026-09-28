use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_jwt::verify_jwt;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn reports_the_rejection_of_the_signature() {
    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::from_jwks(vec![FixtureKey::generate(Curve::P256, "published").jwk()])
    else {
        panic!("the fixture key set is accepted");
    };
    let unpublished = FixtureKey::generate(Curve::P256, "unpublished");
    let token = unpublished.token(&unpublished.header(), &json!({ "exp": 1_000, "iat": 900 }));

    assert!(matches!(
        verify_jwt::<Map<String, Value>>(&key_set, &token, NumericDate::new(950)),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
}
