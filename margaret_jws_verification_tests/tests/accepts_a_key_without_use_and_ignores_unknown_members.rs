use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn accepts_a_key_without_use_and_ignores_unknown_members() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let mut published = serde_json::to_value(key.jwk()).expect("the fixture jwk serializes");
    let members = published.as_object_mut().expect("a jwk is an object");

    members.remove("use");
    members.insert("x5t".to_string(), Value::String("ignored".to_string()));

    let KeySetParsing::Accepted(key_set) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };

    assert!(matches!(
        key_set.verify(&key.token(&key.header(), &json!({}))),
        JwsVerification::Verified(_)
    ));
}
