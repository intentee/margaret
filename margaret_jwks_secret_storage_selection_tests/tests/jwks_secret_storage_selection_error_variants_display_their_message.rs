use margaret_jwks_secret_storage_selection::jwks_secret_storage_selection_error::JwksSecretStorageSelectionError;

#[test]
fn jwks_secret_storage_selection_error_variants_display_their_message() {
    assert!(
        JwksSecretStorageSelectionError::FileOptionWithoutFileStorage
            .to_string()
            .contains("--jwks-secret-file")
    );
    assert!(
        JwksSecretStorageSelectionError::FileStorageRequiresPath
            .to_string()
            .contains("--jwks-secret-file")
    );
}
