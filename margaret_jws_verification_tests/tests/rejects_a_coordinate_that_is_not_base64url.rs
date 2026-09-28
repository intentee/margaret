use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_coordinate_that_is_not_base64url() {
    let mut key = FixtureKey::generate(Curve::P256, "kid").ec_jwk();

    key.y = "not base64 @@@".to_string();

    let parsing = VerificationKeySet::from_jwks(vec![Jwk::Ec(key)]);

    assert!(matches!(
        parsing,
        KeySetParsing::Rejected(KeySetRejection::Key {
            rejection: JwkRejection::CoordinateBase64 { .. },
            ..
        })
    ));
}
