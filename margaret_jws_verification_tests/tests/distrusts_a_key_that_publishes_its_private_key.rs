use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::disclosed_key::DisclosedKey;
use margaret_jws_verification::key_disclosure::KeyDisclosure;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn distrusts_a_key_that_publishes_its_private_key() {
    let mut published = serde_json::to_value(FixtureKey::generate(Curve::P256, "kid").jwk())
        .expect("the fixture jwk serializes");

    published
        .as_object_mut()
        .expect("a jwk is an object")
        .insert("d".to_string(), json!("cHJpdmF0ZQ"));

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { disclosed_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        disclosed_keys.as_slice(),
        [DisclosedKey {
            disclosure: KeyDisclosure::PrivateKey,
            index: 0
        }]
    ));
}
