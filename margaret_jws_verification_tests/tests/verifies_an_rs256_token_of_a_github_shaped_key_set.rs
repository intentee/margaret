use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[test]
fn verifies_an_rs256_token_of_a_github_shaped_key_set() {
    let key = FixtureRsaKey::load("38826b17");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.insert("x5c".to_string(), json!(["MIIDKzCCAhOgAwIBAgIU"]));
    members.insert("x5t".to_string(), Value::String("cxjM2VNf".to_string()));

    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };
    let header = json!({ "typ": "JWT", "alg": "RS256", "x5t": "cxjM2VNf", "kid": "38826b17" });

    assert!(matches!(
        key_set.verify(&key.token(&header, &json!({ "sub": "repo:octo-org/octo-repo" }))),
        JwsVerification::Verified(_)
    ));
}
