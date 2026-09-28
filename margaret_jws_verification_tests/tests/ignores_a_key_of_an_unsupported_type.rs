use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[test]
fn ignores_a_key_of_an_unsupported_type() {
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(
            json!({ "keys": [{ "kty": "oct", "kid": "kid", "k": "c2VjcmV0" }] })
                .to_string()
                .as_bytes(),
        )
    else {
        panic!("the key set is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::UnsupportedKeyType { kty }
        }] if kty == "oct"
    ));
}
