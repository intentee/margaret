use serde_json::json;

use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn rejects_an_rs256_signature_over_another_input() {
    let key = FixtureRsaKey::load("kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };
    let signed = signing_input(&key.header(), &json!({ "sub": "signed" }));
    let presented = signing_input(&key.header(), &json!({ "sub": "presented" }));

    assert!(matches!(
        key_set.verify(&signed_token(&presented, &key.signature(&signed))),
        JwsVerification::Rejected(JwsRejection::RsaSignatureMismatch { .. })
    ));
}
