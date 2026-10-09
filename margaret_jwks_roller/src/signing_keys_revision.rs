use zeroize::Zeroizing;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

use crate::roller_error::RollerError;
use crate::signing_keys_document::SigningKeysDocument;

#[derive(Clone)]
pub struct SigningKeysRevision {
    pub document: SigningKeysDocument,
    pub generation: SigningKeysGeneration,
}

impl SigningKeysRevision {
    /// # Errors
    ///
    /// Returns `RollerError::DocumentSerialization` when the secret cannot be serialized.
    pub fn from_secret(secret: &JwksSecret) -> Result<Self, RollerError> {
        serde_json::to_string(&PersistedJwksSecret::from_secret(secret))
            .map(|json| Self {
                document: SigningKeysDocument::new(Zeroizing::new(json)),
                generation: secret.generation(),
            })
            .map_err(RollerError::DocumentSerialization)
    }
}
