use std::path::Path;
use std::str::FromStr;

use margaret_jwks_secret_storage_selection::jwks_secret_storage_uri::JwksSecretStorageUri;

#[test]
fn jwks_secret_storage_uri_parses_a_file_path() {
    assert!(matches!(
        JwksSecretStorageUri::from_str("file:/var/lib/app/jwks.json"),
        Ok(JwksSecretStorageUri::File { path }) if path == Path::new("/var/lib/app/jwks.json")
    ));
}
