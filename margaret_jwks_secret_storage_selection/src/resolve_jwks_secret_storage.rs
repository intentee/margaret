use std::path::PathBuf;
use std::sync::Arc;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

use crate::jwks_secret_storage_kind::JwksSecretStorageKind;
use crate::jwks_secret_storage_selection_error::JwksSecretStorageSelectionError;

pub fn resolve_jwks_secret_storage(
    kind: JwksSecretStorageKind,
    file: Option<PathBuf>,
) -> Result<Arc<dyn JwksSecretStorage>, JwksSecretStorageSelectionError> {
    match (kind, file) {
        (JwksSecretStorageKind::Memory, None) => Ok(Arc::new(MemoryJwksSecretStorage)),
        (JwksSecretStorageKind::Memory, Some(_)) => {
            Err(JwksSecretStorageSelectionError::FileOptionWithoutFileStorage)
        }
        (JwksSecretStorageKind::File, Some(path)) => Ok(Arc::new(FileJwksSecretStorage::new(path))),
        (JwksSecretStorageKind::File, None) => {
            Err(JwksSecretStorageSelectionError::FileStorageRequiresPath)
        }
    }
}
