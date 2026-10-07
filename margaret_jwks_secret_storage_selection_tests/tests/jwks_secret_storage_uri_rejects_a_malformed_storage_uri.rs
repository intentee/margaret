use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;
use margaret_storage_uri::storage_uri_error::StorageUriError;

#[test]
fn jwks_secret_storage_uri_rejects_a_malformed_storage_uri() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("vault://secrets.localhost/jwks"),
        Err(JwksSecretStorageUriError::StorageUri(StorageUriError::UnknownScheme { scheme }))
            if scheme == "vault"
    ));
}
