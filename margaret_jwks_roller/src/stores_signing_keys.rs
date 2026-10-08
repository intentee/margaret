use async_trait::async_trait;

use crate::signing_keys_document::SigningKeysDocument;
use crate::stored_signing_keys::StoredSigningKeys;

#[async_trait]
pub trait StoresSigningKeys: Send + Sync {
    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys>;

    /// # Errors
    ///
    /// Returns an error when the application cannot reach its storage.
    async fn store_signing_keys(&self, document: &SigningKeysDocument) -> anyhow::Result<()>;
}
