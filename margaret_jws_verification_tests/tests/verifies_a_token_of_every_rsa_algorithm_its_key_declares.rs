use aws_lc_rs::signature::RSA_PKCS1_SHA256;
use aws_lc_rs::signature::RSA_PKCS1_SHA384;
use aws_lc_rs::signature::RSA_PKCS1_SHA512;
use aws_lc_rs::signature::RSA_PSS_SHA256;
use aws_lc_rs::signature::RSA_PSS_SHA384;
use aws_lc_rs::signature::RSA_PSS_SHA512;
use aws_lc_rs::signature::RsaEncoding;
use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

fn verifies_a_token_under(algorithm: JwsAlgorithm, padding: &'static dyn RsaEncoding) -> bool {
    let key = FixtureRsaKey::load("kid");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");

    published
        .as_object_mut()
        .expect("a jwk is an object")
        .insert("alg".to_string(), json!(algorithm.wire_name()));

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };
    let signing_input = signing_input(
        &json!({ "alg": algorithm.wire_name(), "kid": "kid" }),
        &json!({}),
    );
    let token = signed_token(&signing_input, &key.signature_of(padding, &signing_input));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    matches!(key_set.verify(&jws), JwsVerification::Verified(_))
}

#[test]
fn verifies_a_token_of_every_rsa_algorithm_its_key_declares() {
    assert!(verifies_a_token_under(
        JwsAlgorithm::Rs256,
        &RSA_PKCS1_SHA256
    ));
    assert!(verifies_a_token_under(
        JwsAlgorithm::Rs384,
        &RSA_PKCS1_SHA384
    ));
    assert!(verifies_a_token_under(
        JwsAlgorithm::Rs512,
        &RSA_PKCS1_SHA512
    ));
    assert!(verifies_a_token_under(JwsAlgorithm::Ps256, &RSA_PSS_SHA256));
    assert!(verifies_a_token_under(JwsAlgorithm::Ps384, &RSA_PSS_SHA384));
    assert!(verifies_a_token_under(JwsAlgorithm::Ps512, &RSA_PSS_SHA512));
}
