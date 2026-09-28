use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn rejects_a_modulus_above_8192_bits() {
    let mut published = FixtureRsaKey::load("kid").rsa_jwk();
    let mut modulus = vec![0x01];

    modulus.resize(1025, 0xff);
    published.n = Base64UrlUnpadded::encode_string(&modulus);

    assert!(matches!(
        VerificationKeySet::from_jwks(vec![Jwk::Rsa(published)]),
        KeySetParsing::Rejected(KeySetRejection::Key {
            index: 0,
            rejection: JwkRejection::ModulusSize { bits: 8193 }
        })
    ));
}
