use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;

#[must_use]
pub fn unrolled_store() -> JwksSecretStore {
    JwksSecretStore::new(JwksSecretHolder::default())
}
