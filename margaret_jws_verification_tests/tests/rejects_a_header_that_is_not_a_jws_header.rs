use base64ct::Base64UrlUnpadded;
use base64ct::Encoding;

use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;

#[test]
fn rejects_a_header_that_is_not_a_jws_header() {
    let token = format!("{}.e30.AAAA", Base64UrlUnpadded::encode_string(b"not json"));

    assert!(matches!(
        CompactJws::parse(&token),
        CompactJwsParsing::Rejected(JwsRejection::HeaderMalformed { .. })
    ));
}
