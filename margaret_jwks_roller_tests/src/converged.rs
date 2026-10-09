use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;

/// # Panics
///
/// Panics when a published key set cannot be serialized.
#[must_use]
pub fn converged(secrets: &[Arc<JwksSecret>]) -> bool {
    secrets.windows(2).all(|pair| {
        pair[0].generation() == pair[1].generation()
            && pair[0].current().kid() == pair[1].current().kid()
            && pair[0].next().kid() == pair[1].next().kid()
            && serde_json::to_vec(pair[0].public_jwks()).expect("the published set serializes")
                == serde_json::to_vec(pair[1].public_jwks()).expect("the published set serializes")
    })
}
