use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn file_storage_reports_a_write_error_when_the_parent_is_missing() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("missing").join("jwks.json");
    let storage = FileJwksSecretStorage::new(path);

    let Err(RollerError::SecretPersist { source }) = storage.persist(&sample_secret()) else {
        panic!("writing into a missing directory fails");
    };
    let backend = source
        .downcast_ref::<FileJwksSecretStorageError>()
        .expect("the persist error carries a file storage source");

    assert!(matches!(backend, FileJwksSecretStorageError::Write { .. }));
}
