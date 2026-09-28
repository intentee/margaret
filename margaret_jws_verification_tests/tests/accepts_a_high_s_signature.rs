use p256::ecdsa::Signature;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn accepts_a_high_s_signature() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };

    let signing_input = signing_input(&key.header(), &json!({}));
    let low_s =
        Signature::from_slice(&key.signature(&signing_input)).expect("the signature parses");
    let normalized = low_s.normalize_s().unwrap_or(low_s);
    let (r, s) = normalized.split_scalars();
    let high_s = Signature::from_scalars(r, -s).expect("negating s yields a signature");

    assert!(high_s.normalize_s().is_some());
    assert!(matches!(
        key_set.verify(&signed_token(&signing_input, &high_s.to_bytes())),
        JwsVerification::Verified(_)
    ));
}
