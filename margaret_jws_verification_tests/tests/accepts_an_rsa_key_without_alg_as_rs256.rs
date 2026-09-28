use serde_json::json;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jwk::Jwk;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn accepts_an_rsa_key_without_alg_as_rs256() {
    let key = FixtureRsaKey::load("kid");
    let mut published = key.rsa_jwk();

    published.alg = None;

    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![Jwk::Rsa(published)])
    else {
        panic!("the key set is accepted");
    };

    let token = key.token(&key.header(), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
