use anyhow::Result;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn persisted_jwks_secret_rejects_an_rsa_key_sharing_the_key_id_of_an_ec_key() -> Result<()> {
    let mut document = serde_json::to_value(PersistedJwksSecret::from_secret(&JwksSecret::fresh(
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )?))?;

    document["rsa"]["next"]["kid"] = document["current"]["signing"]["kid"].clone();

    assert!(matches!(
        serde_json::from_value::<PersistedJwksSecret>(document)?
            .into_secret(&FixtureRsaSigningKeys::default()),
        Err(JwksKeyError::DuplicateKeyId { .. })
    ));

    Ok(())
}
