use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::key_usage_rejection::KeyUsageRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn ignores_a_key_without_use_among_encryption_keys() {
    let encryption_key = FixtureRsaKey::load("enc-kid").rsa_jwk();
    let mut signing_key = serde_json::to_value(FixtureKey::generate(Curve::P256, "sig-kid").jwk())
        .expect("the fixture jwk serializes");

    signing_key
        .as_object_mut()
        .expect("a jwk is an object")
        .remove("use");

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [
        { "kty": "RSA", "use": "enc", "alg": "RSA-OAEP", "kid": "enc-kid", "n": encryption_key.n, "e": encryption_key.e },
        signing_key,
    ] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [
            IgnoredKey {
                index: 0,
                reason: IgnoredKeyReason::Usage(KeyUsageRejection::EncryptionUse)
            },
            IgnoredKey {
                index: 1,
                reason: IgnoredKeyReason::Usage(KeyUsageRejection::MissingUseAmongEncryptionKeys)
            }
        ]
    ));
}
