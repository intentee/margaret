use std::fs;
use std::os::unix::fs::PermissionsExt;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_persists_with_owner_only_permissions() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    let storage = FileJwksSecretStorage::new(path.clone());

    storage
        .persist(&sample_secret())
        .expect("the secret can be persisted");

    let mode = fs::metadata(&path)
        .expect("the persisted secret file exists")
        .permissions()
        .mode();

    assert_eq!(mode & 0o777, 0o600);
}
