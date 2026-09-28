use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn rejects_a_header_whose_type_is_not_a_string() {
    let key = FixtureKey::generate(Curve::P256, "kid");
    let token = key.token(
        &json!({ "alg": "ES256", "kid": "kid", "typ": 7 }),
        &json!({}),
    );

    assert!(matches!(
        CompactJws::parse(&token),
        CompactJwsParsing::Rejected(JwsRejection::HeaderMalformed { .. })
    ));
}
