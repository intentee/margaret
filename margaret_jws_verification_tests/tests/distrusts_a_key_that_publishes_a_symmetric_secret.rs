use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::disclosed_key::DisclosedKey;
use margaret_jws_verification::key_disclosure::KeyDisclosure;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[test]
fn distrusts_a_key_that_publishes_a_symmetric_secret() {
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { disclosed_keys, .. }) =
        VerificationKeySet::parse(
            json!({ "keys": [{ "kty": "oct", "kid": "kid", "k": "c2VjcmV0" }] })
                .to_string()
                .as_bytes(),
        )
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        disclosed_keys.as_slice(),
        [DisclosedKey {
            disclosure: KeyDisclosure::SymmetricKey,
            index: 0
        }]
    ));
}
