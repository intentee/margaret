use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;

use crate::fixture_issuance::fixture_issuance;
use crate::rolled_secrets::rolled_secrets;

#[must_use]
pub fn rolled_store(secret: JwksSecret) -> JwksSecretStore {
    JwksSecretStore::create(rolled_secrets(secret), fixture_issuance())
}
