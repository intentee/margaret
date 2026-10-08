use async_trait::async_trait;

use margaret_jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret_jwks_roller::stored_signing_keys::StoredSigningKeys;
use margaret_jwks_roller::stores_signing_keys::StoresSigningKeys;

use crate::storage_backend_error::StorageBackendError;

pub struct UnreachableSigningKeys;

#[async_trait]
impl StoresSigningKeys for UnreachableSigningKeys {
    async fn load_signing_keys(&self) -> anyhow::Result<StoredSigningKeys> {
        Err(StorageBackendError::Unreachable.into())
    }

    async fn store_signing_keys(&self, _document: &SigningKeysDocument) -> anyhow::Result<()> {
        Err(StorageBackendError::Unreachable.into())
    }
}
