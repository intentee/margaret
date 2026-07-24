use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;

#[test]
fn jwks_secret_storage_uri_parses_the_memory_scheme() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("memory"),
        Ok(JwksSecretStorageUri::Memory)
    ));
    assert!(matches!(
        JwksSecretStorageUri::from_str("memory:"),
        Ok(JwksSecretStorageUri::Memory)
    ));
}
