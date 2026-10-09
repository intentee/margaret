use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_material_rejection::KeyMaterialRejection;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_octet_key_pair::FixtureOctetKeyPair;

#[test]
fn ignores_an_octet_key_pair_on_an_unsupported_curve() {
    let mut published = FixtureOctetKeyPair::generate("kid").jwk("EdDSA");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert("crv".to_string(), json!("Ed448"));

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey { index: 0, reason: IgnoredKeyReason::Material(KeyMaterialRejection::UnsupportedOctetKeyPairCurve { crv }) }] if crv == "Ed448"
    ));
}
