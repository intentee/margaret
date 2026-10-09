use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

/// # Panics
///
/// Panics when the roller cannot roll its first secret.
pub async fn fixture_roller() -> Arc<JwksRoller> {
    Arc::new(
        JwksRoller::create(
            Arc::new(FixtureSigningKeys::empty()),
            Arc::new(FixtureRsaSigningKeys::default()),
        )
        .await
        .expect("the roller rolls its first secret"),
    )
}
