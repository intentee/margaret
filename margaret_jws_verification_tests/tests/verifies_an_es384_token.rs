use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn verifies_an_es384_token() {
    let key = FixtureKey::generate(Curve::P384, "kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };

    assert!(matches!(
        key_set.verify(&key.token(&key.header(), &json!({}))),
        JwsVerification::Verified(_)
    ));
}
