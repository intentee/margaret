use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_material_rejection::KeyMaterialRejection;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn ignores_coordinates_that_are_not_a_curve_point() {
    let mut published = serde_json::to_value(FixtureKey::generate(Curve::P256, "kid").jwk())
        .expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    let x = members
        .get("x")
        .cloned()
        .expect("the fixture key has an x coordinate");

    members.insert("y".to_string(), x);

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::Material(KeyMaterialRejection::InvalidPoint { .. })
        }]
    ));
}
