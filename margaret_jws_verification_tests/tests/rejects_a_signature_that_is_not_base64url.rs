use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn rejects_a_signature_that_is_not_base64url() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let token = format!("{}.!!!", signing_input(&key.header(), &json!({})));

    assert!(matches!(
        CompactJws::parse(&token),
        CompactJwsParsing::Rejected(JwsRejection::SignatureBase64 { .. })
    ));
}
