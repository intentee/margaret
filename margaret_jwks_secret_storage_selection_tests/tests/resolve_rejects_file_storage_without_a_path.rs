use margaret_jwks_secret_storage_selection::jwks_secret_storage_kind::JwksSecretStorageKind;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_selection_error::JwksSecretStorageSelectionError;
use margaret_jwks_secret_storage_selection::resolve_jwks_secret_storage::resolve_jwks_secret_storage;

#[test]
fn resolve_rejects_file_storage_without_a_path() {
    let selection = resolve_jwks_secret_storage(JwksSecretStorageKind::File, None);

    let Err(error) = selection else {
        panic!("file storage without a path is rejected");
    };

    assert!(matches!(
        error,
        JwksSecretStorageSelectionError::FileStorageRequiresPath
    ));
}
