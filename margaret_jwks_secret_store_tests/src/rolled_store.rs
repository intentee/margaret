use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;

#[must_use]
pub fn rolled_store(secret: JwksSecret) -> JwksSecretStore {
    let holder = JwksSecretHolder::default();

    holder.set(Some(Arc::new(secret)));

    JwksSecretStore::new(holder)
}
