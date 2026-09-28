use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn verifies_a_token_of_a_key_set_with_an_ignored_key() {
    let encryption_key = FixtureRsaKey::load("enc-kid").rsa_jwk();
    let key = FixtureKey::generate(Curve::P256, "sig-kid");
    let document = json!({ "keys": [
        { "kty": "RSA", "use": "enc", "alg": "RSA-OAEP", "kid": "enc-kid", "n": encryption_key.n, "e": encryption_key.e },
        serde_json::to_value(key.jwk()).expect("the fixture jwk serializes"),
    ] });

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(document.to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };
    let token = key.token(&key.header(), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
