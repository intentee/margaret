use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn verifies_a_token_of_the_only_published_key_without_a_key_id() {
    let key = FixtureRsaKey::load("kid");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.remove("kid");

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument {
        ignored_keys,
        key_set,
        ..
    }) = VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };
    let token = key.token(
        &json!({ "alg": JwsAlgorithm::Rs256.wire_name() }),
        &json!({}),
    );
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(ignored_keys.is_empty());
    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Verified(verified) if verified.kid.is_none()
    ));
}
