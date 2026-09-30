use std::sync::Arc;

use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[test]
fn jwks_secret_holder_subscription_reads_current() -> Result<()> {
    let secret = Arc::new(JwksSecret::fresh(
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )?);
    let holder = JwksSecretHolder::default();
    holder.set(Some(secret.clone()));

    let mut subscription = holder.subscribe();

    assert!(matches!(subscription.read_current(), Some(current) if Arc::ptr_eq(&current, &secret)));

    Ok(())
}
