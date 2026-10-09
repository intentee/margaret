use async_trait::async_trait;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

use crate::signing_keys_creation::SigningKeysCreation;
use crate::signing_keys_replacement::SigningKeysReplacement;
use crate::signing_keys_revision::SigningKeysRevision;
use crate::stored_signing_keys::StoredSigningKeys;

#[async_trait]
pub trait StoresSigningKeys: Send + Sync {
    /// Loads the stored revision, observing every write that any instance had acknowledged before
    /// the load began.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys>;

    /// Stores the revision only while no revision is stored, atomically across every instance,
    /// and reports `AlreadyCreated` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn create_signing_keys(
        &self,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysCreation>;

    /// Replaces the stored revision only while its generation is `expected`, atomically across
    /// every instance, and reports `Superseded` otherwise.
    ///
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn replace_signing_keys(
        &self,
        expected: SigningKeysGeneration,
        revision: &SigningKeysRevision,
    ) -> anyhow::Result<SigningKeysReplacement>;
}
