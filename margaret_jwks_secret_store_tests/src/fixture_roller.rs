use std::sync::Arc;

use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;

#[must_use]
pub fn fixture_roller() -> Arc<JwksRoller> {
    Arc::new(JwksRoller::create(Arc::new(MemoryJwksSecretStorage)))
}
