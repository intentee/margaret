use serde_json::Value;
use serde_json::json;

use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification::key_id::KeyId;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

const RSA_PUBLIC_KEY: &str = include_str!("../fixtures/rfc7520/figure_3_rsa_public_key.json");
const PAYLOAD_CONTENT: &str = include_str!("../fixtures/rfc7520/figure_7_payload_content.txt");
const JWS_COMPACT_SERIALIZATION: &str =
    include_str!("../fixtures/rfc7520/figure_13_jws_compact_serialization.txt");

#[test]
fn verifies_the_rfc_7520_rs256_signature_example() {
    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { key_set, .. }) =
        VerificationKeySet::parse(
            json!({
                "keys": [serde_json::from_str::<Value>(RSA_PUBLIC_KEY)
                    .expect("the example key is json")],
            })
            .to_string()
            .as_bytes(),
        )
    else {
        panic!("the key set of the example is accepted");
    };
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(JWS_COMPACT_SERIALIZATION) else {
        panic!("the example is a compact jws");
    };

    let JwsVerification::Verified(verified) = key_set.verify(&jws) else {
        panic!("the example signature verifies");
    };

    assert_eq!(
        verified.kid.map(KeyId::as_str),
        Some("bilbo.baggins@hobbiton.example")
    );
    assert_eq!(jws.payload(), PAYLOAD_CONTENT.as_bytes());
}
