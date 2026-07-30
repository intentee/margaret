use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn sample_secret() -> JwksSecret {
    JwksSecret::fresh(Curve::P256).expect("a fresh jwks secret can be generated")
}
