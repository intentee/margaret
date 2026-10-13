use bytes::Bytes;

use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_jwks_keygen::public_jwks::PublicJwks;

/// # Panics
///
/// Panics when the key set cannot be serialized.
#[must_use]
pub fn key_set_document(key: &JwkPair) -> Bytes {
    Bytes::from(
        serde_json::to_vec(&PublicJwks::new(vec![key.public_jwk().clone()]))
            .expect("the key set serializes"),
    )
}
