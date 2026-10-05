use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::parameter_value::ParameterValue;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn exposes_the_algorithm_its_header_declares() {
    let token = signed_token(
        &signing_input(&json!({ "alg": "ES256" }), &json!({ "sub": "subject" })),
        b"signature",
    );
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token is a compact jws");
    };

    assert_eq!(jws.alg(), &ParameterValue::Supported(JwsAlgorithm::Es256));
}
