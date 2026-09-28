use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jws_verification::key_set_parsing::KeySetParsing;
use margaret_jws_verification::verification_key_set::VerificationKeySet;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn published_key_set(secret: &JwksSecret) -> KeySetParsing {
    VerificationKeySet::parse(
        &serde_json::to_vec(secret.public_jwks()).expect("the published key set serializes"),
    )
}
