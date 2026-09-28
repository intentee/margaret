use serde_json::json;

use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::accepted_key_set_document::AcceptedKeySetDocument;
use margaret_jws_verification::ignored_key::IgnoredKey;
use margaret_jws_verification::ignored_key_reason::IgnoredKeyReason;
use margaret_jws_verification::key_set_document_parsing::KeySetDocumentParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jws_verification_tests::fixture_key::FixtureKey;

#[test]
fn ignores_a_key_on_an_unsupported_curve() {
    let key = FixtureKey::generate(Curve::P384, "kid").ec_jwk();
    let document = json!({ "keys": [
        { "kty": "EC", "crv": "P-521", "kid": "kid", "x": key.x, "y": key.y },
    ] });

    let KeySetDocumentParsing::Accepted(AcceptedKeySetDocument { ignored_keys, .. }) =
        VerificationKeySet::parse(document.to_string().as_bytes())
    else {
        panic!("the key set is accepted");
    };

    assert!(matches!(
        ignored_keys.as_slice(),
        [IgnoredKey {
            index: 0,
            reason: IgnoredKeyReason::UnsupportedCurve { crv }
        }] if crv == "P-521"
    ));
}
