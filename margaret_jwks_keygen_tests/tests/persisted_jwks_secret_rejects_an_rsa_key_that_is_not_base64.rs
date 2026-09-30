use anyhow::Result;
use serde_json::Value;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_rejects_an_rsa_key_that_is_not_base64() -> Result<()> {
    let rsa_keys = FixtureRsaSigningKeys::default();
    let mut document = serde_json::to_value(PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        SigningCurve::P256,
        &rsa_keys,
    )?))?;

    document["rsa"]["current"]["pkcs8"] = Value::String("not base64!".to_string());

    assert!(matches!(
        serde_json::from_value::<PersistedJwksSecret>(document)?.into_secret(&rsa_keys),
        Err(JwksKeyError::RsaKeyBase64 { .. })
    ));

    Ok(())
}
