use anyhow::Result;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_rejects_a_corrupt_signing_key() -> Result<()> {
    let persisted = serde_json::to_value(PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )?))?;

    for slot in ["current", "next", "previous"] {
        let mut document = persisted.clone();

        document[slot]["signing"]["pem"] = "not a pkcs8 pem".into();

        assert!(matches!(
            serde_json::from_value::<PersistedJwksSecret>(document)?
                .into_secret(&FixtureRsaSigningKeys::default()),
            Err(JwksKeyError::SigningKeyRejected { .. })
        ));
    }

    Ok(())
}
