use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

use crate::fixture_roller::fixture_roller;

#[must_use]
pub fn rolled_store(secret: JwksSecret) -> JwksSecretStore {
    let roller = fixture_roller();

    roller.jwks_secret_holder().set(Some(Arc::new(secret)));

    JwksSecretStore::create(roller, Arc::new(fixture_issuance()))
}
