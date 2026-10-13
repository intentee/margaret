use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn rejects_a_signature_of_the_wrong_length() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetAssembly::Assembled(key_set) =
        VerificationKeySet::assemble(vec![key.verification_key()])
    else {
        panic!("the fixture key set is accepted");
    };

    let token = format!("{}.AAAA", signing_input(&key.header(), &json!({})));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Rejected(JwsRejection::SignatureLength {
            expected: 64,
            found: 3
        })
    ));
}
