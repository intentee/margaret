use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn accepts_an_8192_bit_modulus() {
    let mut published =
        serde_json::to_value(FixtureRsaKey::load("kid").jwk()).expect("the fixture jwk serializes");

    published
        .as_object_mut()
        .expect("a jwk is an object")
        .insert(
            "n".to_string(),
            json!(Base64UrlUnpadded::encode_string(&[0xff; 1024])),
        );

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(ignored_keys.is_empty());
}
