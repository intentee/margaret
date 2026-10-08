use serde_json::from_value;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;

#[test]
fn jwks_secret_roll_reports_an_exhausted_generation() {
    let secret =
        from_value::<PersistedJwksSecret>(persisted_document(&fresh_secret(SigningCurve::P256)))
            .expect("the document has the persisted shape")
            .into_secret(
                SigningKeysGeneration::new(u64::MAX),
                signing_key_retention(),
            )
            .expect("the document restores");

    assert!(matches!(
        secret.rolled(&FixtureRsaSigningKeys::default(), secret.rolled_at()),
        Err(JwksKeyError::GenerationExhausted { generation }) if generation == SigningKeysGeneration::new(u64::MAX)
    ));
}
