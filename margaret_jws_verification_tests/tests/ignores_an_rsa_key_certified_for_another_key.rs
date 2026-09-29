use base64ct::Base64;
use base64ct::Encoding;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::certificate_rejection::CertificateRejection;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn ignores_an_rsa_key_certified_for_another_key() {
    let key = FixtureRsaKey::load("kid");
    let certificate = FixtureKey::generate(Curve::P256, "other").certificate();
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert(
        "x5c".to_string(),
        json!([Base64::encode_string(&certificate)]),
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
            reason: IgnoredKeyReason::Certificate(CertificateRejection::KeyMismatch)
        }]
    ));
}
