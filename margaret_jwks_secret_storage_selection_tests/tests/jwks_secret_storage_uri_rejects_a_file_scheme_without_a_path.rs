use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[test]
fn jwks_secret_storage_uri_rejects_a_file_scheme_without_a_path() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("file"),
        Err(JwksSecretStorageUriError::FileRequiresPath)
    ));
    assert!(matches!(
        JwksSecretStorageUri::from_str("file:"),
        Err(JwksSecretStorageUriError::FileRequiresPath)
    ));
}
