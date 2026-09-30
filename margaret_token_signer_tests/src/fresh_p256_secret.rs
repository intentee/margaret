use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;

#[must_use]
/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub fn fresh_p256_secret() -> JwksSecret {
    JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh p256 secret is generated")
}
