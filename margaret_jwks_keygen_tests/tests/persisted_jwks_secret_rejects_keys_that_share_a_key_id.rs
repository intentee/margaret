use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jws_verification::key_set_rejection::KeySetRejection;

#[test]
fn persisted_jwks_secret_rejects_keys_that_share_a_key_id() -> Result<()> {
    let mut document = serde_json::to_value(PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        Curve::P256,
    )?))?;

    document["next"]["signing"]["kid"] = document["current"]["signing"]["kid"].clone();

    assert!(matches!(
        serde_json::from_value::<PersistedJwksSecret>(document)?.into_secret(),
        Err(JwksKeyError::KeySetRejected {
            rejection: KeySetRejection::DuplicateKeyId { .. }
        })
    ));

    Ok(())
}
