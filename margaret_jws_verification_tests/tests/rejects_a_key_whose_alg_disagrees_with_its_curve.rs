use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_key_whose_alg_disagrees_with_its_curve() {
    let mut key = FixtureKey::generate(Curve::P256, "kid").ec_jwk();

    key.alg = Some(Curve::P384.algorithm());

    let parsing = VerificationKeySet::from_jwks(vec![Jwk::Ec(key)]);

    assert!(matches!(
        parsing,
        KeySetParsing::Rejected(KeySetRejection::Key {
            index: 0,
            rejection: JwkRejection::AlgorithmMismatch { .. }
        })
    ));
}
