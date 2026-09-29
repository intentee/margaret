use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::duplicate_key_id::DuplicateKeyId;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::key_set_document_rejection::KeySetDocumentRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_published_set_with_a_duplicate_key_id() {
    let document = json!({ "keys": [
        FixtureKey::generate(Curve::P256, "same").jwk(),
        FixtureKey::generate(Curve::P256, "same").jwk(),
    ] });

    assert!(matches!(
        VerificationKeySet::parse(document.to_string().as_bytes()),
        KeySetDocumentParsing::Rejected(KeySetDocumentRejection::DuplicateKeyId(DuplicateKeyId { kid }))
            if kid.as_str() == "same"
    ));
}
