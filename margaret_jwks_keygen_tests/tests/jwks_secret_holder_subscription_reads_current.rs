use std::sync::Arc;

use anyhow::Result;

use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;

#[test]
fn jwks_secret_holder_subscription_reads_current() -> Result<()> {
    let secret = Arc::new(JwksSecret::fresh(Curve::P256)?);
    let holder = JwksSecretHolder::default();
    holder.set(Some(secret.clone()));

    let mut subscription = holder.subscribe();

    assert!(matches!(subscription.read_current(), Some(current) if Arc::ptr_eq(&current, &secret)));

    Ok(())
}
