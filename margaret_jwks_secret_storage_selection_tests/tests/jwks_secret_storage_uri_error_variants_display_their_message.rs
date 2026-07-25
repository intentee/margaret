use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[test]
fn jwks_secret_storage_uri_error_variants_display_their_message() {
    assert!(
        JwksSecretStorageUriError::FileRequiresPath
            .to_string()
            .contains("requires a path")
    );
    assert!(
        JwksSecretStorageUriError::MemoryTakesNoPath {
            path: "/oops".to_string(),
        }
        .to_string()
        .contains("/oops")
    );
    assert!(
        JwksSecretStorageUriError::UnknownScheme {
            scheme: "vault".to_string(),
        }
        .to_string()
        .contains("vault")
    );
}
