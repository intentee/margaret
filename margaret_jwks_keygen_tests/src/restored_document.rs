use serde_json::Value;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::signing_key_retention::signing_key_retention;

/// # Errors
///
/// Returns `JwksKeyError` when the document's keys cannot be restored.
///
/// # Panics
///
/// Panics when the document does not have the persisted shape.
pub fn restored_document(document: Value) -> Result<JwksSecret, JwksKeyError> {
    serde_json::from_value::<PersistedJwksSecret>(document)
        .expect("the document has the persisted shape")
        .into_secret(SigningKeysGeneration::FIRST, signing_key_retention())
}
