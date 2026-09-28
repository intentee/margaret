use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_an_unsigned_token() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetParsing::Accepted(key_set) = VerificationKeySet::from_jwks(vec![key.jwk()]) else {
        panic!("the fixture key set is accepted");
    };

    let token = format!(
        "{}.",
        FixtureKey::signing_input(&json!({ "alg": "none", "kid": "kid" }), &json!({}))
    );
    assert!(matches!(
        key_set.verify(&token),
        JwsVerification::Rejected(JwsRejection::UnsupportedAlgorithm { alg }) if alg == "none"
    ));
}
