use std::fs;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_persist_leaves_no_temporary_file_behind() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    let storage = FileJwksSecretStorage::new(path.clone());

    storage
        .persist(&sample_secret())
        .expect("the secret can be persisted");
    storage
        .persist(&sample_secret())
        .expect("the secret can be persisted over the previous document");

    let entries: Vec<_> = fs::read_dir(directory.path())
        .expect("the storage directory can be listed")
        .map(|entry| entry.expect("the directory entry can be read").path())
        .collect();

    assert_eq!(entries, vec![path]);
}
