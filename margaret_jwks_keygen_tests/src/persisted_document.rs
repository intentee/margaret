use serde_json::Value;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn persisted_document(secret: &JwksSecret) -> Value {
    serde_json::to_value(PersistedJwksSecret::from_secret(secret))
        .expect("the persisted secret serializes")
}
