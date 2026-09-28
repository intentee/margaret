use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn verifies_an_rs256_token_of_a_key_set_with_certificate_members() {
    let key = FixtureRsaKey::load("rsa-kid");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert("x5c".to_string(), json!(["Y2VydGlmaWNhdGU="]));
    members.insert(
        "x5t".to_string(),
        Value::String("dGh1bWJwcmludA".to_string()),
    );

    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };
    let header = json!({ "typ": "JWT", "alg": "RS256", "x5t": "dGh1bWJwcmludA", "kid": "rsa-kid" });

    let token = key.token(&header, &json!({ "sub": "service-account" }));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));
}
