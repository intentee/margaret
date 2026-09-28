use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::key_set_rejection::KeySetRejection;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

#[test]
fn rejects_a_document_that_is_not_a_key_set() {
    let parsing = VerificationKeySet::parse(br#"{"issuer":"https://issuer.example"}"#);

    assert!(matches!(
        parsing,
        KeySetParsing::Rejected(KeySetRejection::Malformed { .. })
    ));
}
