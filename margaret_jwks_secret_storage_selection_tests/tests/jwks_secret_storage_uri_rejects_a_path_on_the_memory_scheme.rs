use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[test]
fn jwks_secret_storage_uri_rejects_a_path_on_the_memory_scheme() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("memory:/oops"),
        Err(JwksSecretStorageUriError::MemoryTakesNoPath { path }) if path == "/oops"
    ));
}
