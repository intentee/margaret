use serde_json::json;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;

#[test]
fn persisted_jwks_secret_keeps_no_private_material_of_retired_keys() {
    let secret = rolled_secret(&fresh_secret(SigningCurve::P256));
    let serialized = persisted_document(&secret);
    let ec_retired = &secret.retired()[0];
    let rsa_retired = &secret.rsa().retired()[0];

    assert_eq!(
        serialized["ec"]["retired"],
        json!([{
            "public": ec_retired.public_jwk(),
            "retired_at": ec_retired.retired_at().seconds_since_epoch(),
        }])
    );
    assert_eq!(
        serialized["rsa"]["retired"],
        json!([{
            "public": rsa_retired.public_jwk(),
            "retired_at": rsa_retired.retired_at().seconds_since_epoch(),
        }])
    );
}
