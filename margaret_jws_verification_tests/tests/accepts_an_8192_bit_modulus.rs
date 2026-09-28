use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn accepts_an_8192_bit_modulus() {
    let mut published = FixtureRsaKey::load("kid").rsa_jwk();

    published.n = Base64UrlUnpadded::encode_string(&[0xff; 1024]);

    assert!(matches!(
        VerificationKeySet::from_jwks(vec![Jwk::Rsa(published)]),
        KeySetParsing::Accepted(_)
    ));
}
