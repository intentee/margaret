use std::sync::Arc;

use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn jwks_secret_holder_returns_set_secret() -> Result<()> {
    let secret = Arc::new(JwksSecret::fresh(
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )?);
    let holder = JwksSecretHolder::default();

    assert!(holder.get().is_none());

    holder.set(Some(secret.clone()));

    assert!(matches!(holder.get(), Some(stored) if Arc::ptr_eq(&stored, &secret)));

    let cloned_holder = holder.clone();

    assert!(matches!(cloned_holder.get(), Some(stored) if Arc::ptr_eq(&stored, &secret)));

    Ok(())
}
