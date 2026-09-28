use std::sync::Arc;

use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;

use crate::fixture_roller::fixture_roller;

#[must_use]
pub fn unrolled_store() -> JwksSecretStore {
    JwksSecretStore::create(fixture_roller(), Arc::new(fixture_issuance()))
}
