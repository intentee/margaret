use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;
use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_material_rejection::KeyMaterialRejection;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_octet_key_pair::FixtureOctetKeyPair;

#[test]
fn ignores_an_octet_key_pair_whose_public_key_has_the_wrong_length() {
    let mut published = FixtureOctetKeyPair::generate("kid").jwk("Ed25519");

    published
        .as_object_mut()
        .expect("a jwk is an object")
        .insert(
            "x".to_string(),
            json!(Base64UrlUnpadded::encode_string(&[7; 31])),
        );

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::Material(KeyMaterialRejection::CoordinateLength {
                expected: 32,
                found: 31
            })
        }]
    ));
}
