use zeroize::Zeroizing;

use margaret::framework::active_record::secret_text::SecretText;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

#[derive(Clone)]
pub struct SigningKeysRevision {
    pub document: SecretText,
    pub generation: SigningKeysGeneration,
}

impl SigningKeysRevision {
    /// # Errors
    ///
    /// Returns `serde_json::Error` when the secret cannot be serialized.
    pub fn from_secret(secret: &JwksSecret) -> Result<Self, serde_json::Error> {
        serde_json::to_string(&PersistedJwksSecret::from_secret(secret)).map(|json| Self {
            document: SecretText::new(Zeroizing::new(json)),
            generation: secret.generation(),
        })
    }
}
