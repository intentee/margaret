use std::path::PathBuf;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_kind::JwksSecretStorageKind;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_selection_error::JwksSecretStorageSelectionError;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_rejects_a_file_path_without_file_storage() {
    let selection = resolve_jwks_secret_storage(
        JwksSecretStorageKind::Memory,
        Some(PathBuf::from("/secrets/jwks.json")),
    );

    let Err(error) = selection else {
        panic!("a file path with memory storage is rejected");
    };

    assert!(matches!(
        error,
        JwksSecretStorageSelectionError::FileOptionWithoutFileStorage
    ));
}
