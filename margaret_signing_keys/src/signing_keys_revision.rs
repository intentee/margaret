use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys_encryption::sealed_document::SealedDocument;
use margaret_signing_keys_encryption::signing_keys_encryption_key::SigningKeysEncryptionKey;
use margaret_signing_keys_encryption::wiping_buffer::WipingBuffer;

use crate::signing_keys_associated_data::signing_keys_associated_data;
use crate::signing_keys_error::SigningKeysError;

#[derive(Clone)]
pub struct SigningKeysRevision {
    pub generation: SigningKeysGeneration,
    pub sealed: SealedDocument,
}

impl SigningKeysRevision {
    /// # Errors
    ///
    /// Returns `SigningKeysError::DocumentSerialization` when the secret cannot be serialized, and
    /// `SigningKeysError::DocumentSeal` when its document cannot be sealed.
    pub fn sealed(
        secret: &JwksSecret,
        encryption_key: &SigningKeysEncryptionKey,
    ) -> Result<Self, SigningKeysError> {
        let mut plaintext = WipingBuffer::default();

        serde_json::to_writer(&mut plaintext, &PersistedJwksSecret::from_secret(secret))
            .map_err(SigningKeysError::DocumentSerialization)
            .and_then(|()| {
                encryption_key
                    .seal(
                        plaintext,
                        &signing_keys_associated_data(secret.generation()),
                    )
                    .map_err(SigningKeysError::DocumentSeal)
            })
            .map(|sealed| Self {
                generation: secret.generation(),
                sealed,
            })
    }

    /// # Errors
    ///
    /// Returns `SigningKeysError::DocumentOpen` when the document does not open under the key for
    /// its generation, and `SigningKeysError::DocumentMalformed` when the opened document is not
    /// persisted signing keys.
    pub fn opened(
        self,
        encryption_key: &SigningKeysEncryptionKey,
    ) -> Result<PersistedJwksSecret, SigningKeysError> {
        let Self { generation, sealed } = self;
        let plaintext = encryption_key
            .open(sealed, &signing_keys_associated_data(generation))
            .map_err(SigningKeysError::DocumentOpen)?;

        serde_json::from_slice(&plaintext).map_err(SigningKeysError::DocumentMalformed)
    }
}
