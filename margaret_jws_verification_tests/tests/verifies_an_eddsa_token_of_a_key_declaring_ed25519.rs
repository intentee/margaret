use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_octet_key_pair::FixtureOctetKeyPair;

#[test]
fn verifies_an_eddsa_token_of_a_key_declaring_ed25519() {
    let key = FixtureOctetKeyPair::generate("kid");
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(
            json!({ "keys": [key.jwk("Ed25519")] })
                .to_string()
                .as_bytes(),
        )
    else {
        panic!("the key set document is accepted");
    };
    let token = key.token("EdDSA", &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
