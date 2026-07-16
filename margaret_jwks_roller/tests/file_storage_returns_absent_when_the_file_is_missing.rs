use tempfile::tempdir;

use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

#[test]
fn file_storage_returns_absent_when_the_file_is_missing() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(directory.path().join("missing.json"));

    assert!(matches!(storage.load(), Ok(LoadedSecret::Absent)));
}
