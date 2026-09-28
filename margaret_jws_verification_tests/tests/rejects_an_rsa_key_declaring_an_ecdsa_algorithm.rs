use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn rejects_an_rsa_key_declaring_an_ecdsa_algorithm() {
    let mut published = FixtureRsaKey::load("kid").rsa_jwk();

    published.alg = Some(JwsAlgorithm::Es256);

    assert!(matches!(
        VerificationKeySet::from_jwks(vec![Jwk::Rsa(published)]),
        KeySetParsing::Rejected(KeySetRejection::Key {
            index: 0,
            rejection: JwkRejection::AlgorithmMismatch {
                declared: JwsAlgorithm::Es256,
                implied: JwsAlgorithm::Rs256,
            }
        })
    ));
}
