use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jws_verification_tests::signed_token::signed_token;
use margaret_jws_verification_tests::signing_input::signing_input;

#[test]
fn rejects_an_rs256_signature_over_another_input() {
    let key = FixtureRsaKey::load("kid");
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(json!({ "keys": [key.jwk()] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };
    let signed = signing_input(&key.header(), &json!({ "sub": "signed" }));
    let presented = signing_input(&key.header(), &json!({ "sub": "presented" }));

    let token = signed_token(&presented, &key.signature(&signed));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(
        key_set.verify(&jws),
        JwsVerification::Rejected(JwsRejection::RsaSignatureMismatch { .. })
    ));
}
