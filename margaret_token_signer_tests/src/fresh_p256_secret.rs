use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn fresh_p256_secret() -> JwksSecret {
    JwksSecret::fresh(Curve::P256).expect("a fresh p256 secret is generated")
}
