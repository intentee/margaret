use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;

#[must_use]
pub fn fresh_p256_secret() -> JwksSecret {
    JwksSecret::fresh(Curve::P256).expect("a fresh p256 secret is generated")
}
