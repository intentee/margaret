use serde_json::json;

use margaret_jws_verification::jwk_rejection::JwkRejection;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[test]
fn rejects_a_key_of_an_unsupported_type() {
    let parsing = VerificationKeySet::parse(
        json!({ "keys": [{ "kty": "oct", "kid": "kid", "k": "c2VjcmV0" }] })
            .to_string()
            .as_bytes(),
    );

    assert!(matches!(
        parsing,
        KeySetParsing::Rejected(KeySetRejection::Key {
            index: 0,
            rejection: JwkRejection::Malformed { .. }
        })
    ));
}
