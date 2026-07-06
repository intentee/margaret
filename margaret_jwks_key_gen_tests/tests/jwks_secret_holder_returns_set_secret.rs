use std::sync::Arc;

use anyhow::Result;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;

#[test]
fn jwks_secret_holder_returns_set_secret() -> Result<()> {
    let secret = Arc::new(JwksSecret::fresh(Curve::P256)?);
    let holder = JwksSecretHolder::default();

    assert!(holder.get().is_none());

    holder.set(Some(secret.clone()));

    assert!(matches!(holder.get(), Some(stored) if Arc::ptr_eq(&stored, &secret)));

    let cloned_holder = holder.clone();

    assert!(matches!(cloned_holder.get(), Some(stored) if Arc::ptr_eq(&stored, &secret)));

    Ok(())
}
