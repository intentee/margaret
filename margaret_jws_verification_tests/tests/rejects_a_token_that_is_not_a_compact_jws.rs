use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;

#[test]
fn rejects_a_token_that_is_not_a_compact_jws() {
    for token in ["single", "two.segments", "four.dotted.jws.segments"] {
        assert!(matches!(
            CompactJws::parse(token),
            CompactJwsParsing::Rejected(JwsRejection::NotCompactJws)
        ));
    }
}
