use aws_lc_rs::signature::RSA_PSS_SHA256;
use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn rejects_a_ps256_token_of_an_rsa_key_without_alg() {
    let key = FixtureRsaKey::load("kid");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");

    published
        .as_object_mut()
        .expect("a jwk is an object")
        .remove("alg");

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };
    let signing_input = signing_input(&json!({ "alg": "PS256", "kid": "kid" }), &json!({}));
    let token = signed_token(
        &signing_input,
        &key.signature_of(&RSA_PSS_SHA256, &signing_input),
    );
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Rejected(JwsRejection::AlgorithmMismatch {
            key: JwsAlgorithm::Rs256,
            token: JwsAlgorithm::Ps256
        })
    ));
}
