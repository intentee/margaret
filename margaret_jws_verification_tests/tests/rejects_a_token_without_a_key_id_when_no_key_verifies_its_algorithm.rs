use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_token_without_a_key_id_when_no_key_verifies_its_algorithm() {
    let signer = FixtureKey::generate(Curve::P256, "signer");
    let KeySetAssembly::Assembled(key_set) = VerificationKeySet::assemble(vec![
        FixtureKey::generate(Curve::P384, "other-algorithm").verification_key(),
    ]) else {
        panic!("the fixture key set is accepted");
    };

    let token = signer.token(&json!({ "alg": "ES256" }), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Rejected(JwsRejection::NoKeyForAlgorithm {
            algorithm: JwsAlgorithm::Es256
        })
    ));
}
