use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn rejects_an_even_rsa_exponent() {
    let mut published = FixtureRsaKey::load("kid").rsa_jwk();

    published.e = Base64UrlUnpadded::encode_string(&65_536_u32.to_be_bytes()[1..]);

    assert!(matches!(
        VerificationKeySet::from_jwks(vec![Jwk::Rsa(published)]),
        KeySetParsing::Rejected(KeySetRejection::Key {
            index: 0,
            rejection: JwkRejection::InvalidRsaKey { .. }
        })
    ));
}
