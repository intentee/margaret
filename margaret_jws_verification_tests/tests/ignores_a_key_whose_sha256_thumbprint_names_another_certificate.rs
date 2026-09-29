use aws_lc_rs::digest;
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
use margaret_jws_verification_tests::certificate_thumbprint::certificate_thumbprint;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn ignores_a_key_whose_sha256_thumbprint_names_another_certificate() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let certificate = key.certificate();
    let other_certificate = FixtureKey::generate(Curve::P256, "other").certificate();
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert(
        "x5c".to_string(),
        json!([Base64::encode_string(&certificate)]),
    );
    members.insert(
        "x5t#S256".to_string(),
        json!(certificate_thumbprint(&digest::SHA256, &other_certificate)),
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
            reason: IgnoredKeyReason::Certificate(CertificateRejection::Sha256ThumbprintMismatch)
        }]
    ));
}
