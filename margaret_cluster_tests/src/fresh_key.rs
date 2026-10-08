use uuid::Uuid;

use margaret_jwks_keygen::ec_signing_key::EcSigningKey;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jws_verification::key_id::KeyId;

/// # Panics
///
/// Panics when the key cannot be generated.
#[must_use]
pub fn fresh_key() -> JwkPair {
    JwkPair::new(
        KeyId::new(Uuid::new_v4().to_string()),
        EcSigningKey::generate(SigningCurve::P256).expect("the signing key generates"),
    )
    .expect("the key pair assembles")
}
