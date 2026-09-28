use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;

#[test]
fn persisted_jwks_secret_rejects_a_corrupt_signing_key() -> Result<()> {
    let persisted = serde_json::to_value(PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        Curve::P256,
    )?))?;

    for slot in ["current", "next", "previous"] {
        let mut document = persisted.clone();

        document[slot]["signing"]["pem"] = "not a pkcs8 pem".into();

        assert!(matches!(
            serde_json::from_value::<PersistedJwksSecret>(document)?.into_secret(),
            Err(JwksKeyError::SigningKeyRejected { .. })
        ));
    }

    Ok(())
}
