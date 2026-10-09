use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn fresh_secret(curve: SigningCurve) -> JwksSecret {
    JwksSecret::fresh(
        curve,
        &FixtureRsaSigningKeys::default(),
        signing_key_retention(),
        NumericDate::new(0),
    )
    .expect("a fresh secret is generated")
}
