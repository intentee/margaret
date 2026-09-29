use aws_lc_rs::digest;
use base64ct::Base64;
use base64ct::Encoding;
use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::certificate_thumbprint::certificate_thumbprint;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn verifies_an_rs256_token_of_a_key_set_with_certificate_members() {
    let key = FixtureRsaKey::load("rsa-kid");
    let certificate = key.certificate();
    let sha1_thumbprint = certificate_thumbprint(&digest::SHA1_FOR_LEGACY_USE_ONLY, &certificate);
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert(
        "x5c".to_string(),
        json!([Base64::encode_string(&certificate)]),
    );
    members.insert("x5t".to_string(), json!(sha1_thumbprint));
    members.insert(
        "x5t#S256".to_string(),
        json!(certificate_thumbprint(&digest::SHA256, &certificate)),
    );

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };
    let header = json!({ "typ": "JWT", "alg": "RS256", "x5t": sha1_thumbprint, "kid": "rsa-kid" });
    let token = key.token(&header, &json!({ "sub": "service-account" }));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
