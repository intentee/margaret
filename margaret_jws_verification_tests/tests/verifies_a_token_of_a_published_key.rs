use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn verifies_a_token_of_a_published_key() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };

    let JwsVerification::Verified(verified) =
        key_set.verify(&key.token(&key.header(), &json!({ "sub": "subject" })))
    else {
        panic!("the token verifies");
    };

    assert_eq!(verified.kid.as_str(), "kid");
    assert_eq!(verified.payload, br#"{"sub":"subject"}"#);
}
