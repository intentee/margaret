use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn ignores_a_key_of_an_unsupported_algorithm() {
    let key = FixtureRsaKey::load("enc-kid").rsa_jwk();
    let document = json!({ "keys": [
        { "kty": "RSA", "use": "enc", "alg": "RSA-OAEP", "kid": "enc-kid", "n": key.n, "e": key.e },
    ] });

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(document.to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::UnsupportedAlgorithm { alg }
        }] if alg == "RSA-OAEP"
    ));
}
