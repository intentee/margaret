use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

use crate::rolled_roller::rolled_roller;

pub async fn rolled_store(secret: JwksSecret) -> JwksSecretStore {
    JwksSecretStore::create(rolled_roller(secret).await, fixture_issuance())
}
