use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;

use crate::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn rolled_secret(secret: &JwksSecret) -> JwksSecret {
    secret
        .rolled(
            &FixtureRsaSigningKeys::default(),
            secret.rolled_at().after(JWKS_ROLL_INTERVAL),
        )
        .expect("the secret rolls")
}
