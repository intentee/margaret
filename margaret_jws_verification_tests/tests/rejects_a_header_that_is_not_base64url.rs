use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;

#[test]
fn rejects_a_header_that_is_not_base64url() {
    assert!(matches!(
        CompactJws::parse("!!!.e30.AAAA"),
        CompactJwsParsing::Rejected(JwsRejection::HeaderBase64 { .. })
    ));
}
