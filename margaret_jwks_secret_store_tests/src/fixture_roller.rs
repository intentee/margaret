use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;

#[must_use]
pub fn fixture_roller() -> Arc<JwksRoller> {
    Arc::new(JwksRoller::create(
        Arc::new(MemoryJwksSecretStorage),
        Arc::new(FixtureRsaSigningKeys::default()),
    ))
}
