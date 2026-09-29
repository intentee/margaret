use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_token_whose_algorithm_disagrees_with_its_key() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetAssembly::Assembled(key_set) =
        VerificationKeySet::assemble(vec![key.verification_key()])
    else {
        panic!("the fixture key set is accepted");
    };

    let token = key.token(&json!({ "alg": "ES384", "kid": "kid" }), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Rejected(JwsRejection::AlgorithmMismatch { .. })
    ));
}
