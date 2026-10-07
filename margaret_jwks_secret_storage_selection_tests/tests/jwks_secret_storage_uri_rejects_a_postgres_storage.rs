use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;
use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri_error::JwksSecretStorageUriError;

#[test]
fn jwks_secret_storage_uri_rejects_a_postgres_storage() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("postgres://app@database.localhost/jwks"),
        Err(JwksSecretStorageUriError::PostgresUnsupported)
    ));
}
