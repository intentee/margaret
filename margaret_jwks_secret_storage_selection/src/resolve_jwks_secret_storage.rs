use std::sync::Arc;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

use crate::jwks_secret_storage_uri::JwksSecretStorageUri;

#[must_use]
pub fn resolve_jwks_secret_storage(uri: JwksSecretStorageUri) -> Arc<dyn JwksSecretStorage> {
    match uri {
        JwksSecretStorageUri::Memory => Arc::new(MemoryJwksSecretStorage),
        JwksSecretStorageUri::File { path } => Arc::new(FileJwksSecretStorage::new(path)),
    }
}
