use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_algorithm_rejection::KeyAlgorithmRejection;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_octet_key_pair::FixtureOctetKeyPair;

#[test]
fn ignores_an_octet_key_pair_declaring_an_ecdsa_algorithm() {
    let published = FixtureOctetKeyPair::generate("kid").jwk("ES256");

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(json!({ "keys": [published] }).to_string().as_bytes())
    else {
        panic!("the key set document is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::Algorithm(KeyAlgorithmRejection::AlgorithmMismatch {
                declared: JwsAlgorithm::Es256,
                implied: JwsAlgorithm::Ed25519
            })
        }]
    ));
}
