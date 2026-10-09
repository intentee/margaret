use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn verifies_a_token_of_an_assembled_key() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let KeySetAssembly::Assembled(key_set) =
        VerificationKeySet::assemble(vec![key.verification_key()])
    else {
        panic!("the fixture key set is accepted");
    };

    let token = key.token(&key.header(), &json!({ "sub": "subject" }));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    let JwsVerification::Verified(verified) = key_set.verify(&jws) else {
        panic!("the token verifies");
    };

    assert_eq!(verified.kid.map(KeyId::as_str), Some("kid"));
    assert_eq!(jws.payload(), br#"{"sub":"subject"}"#);
}
