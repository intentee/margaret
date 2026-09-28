use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::ec_signing_key::EcSigningKey;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_jws_verification::key_id::KeyId;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn fixture_pair(curve: Curve, kid: &str) -> JwkPair {
    JwkPair::new(
        KeyId::new(kid.to_string()),
        EcSigningKey::generate(curve).expect("the fixture key generates"),
    )
    .expect("the fixture pair publishes its key")
}
