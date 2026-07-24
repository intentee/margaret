use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[test]
fn jwks_secret_storage_uri_rejects_an_unknown_scheme() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("vault:/secret"),
        Err(JwksSecretStorageUriError::UnknownScheme { scheme }) if scheme == "vault"
    ));
}
