use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn file_storage_reports_a_read_error_when_the_path_is_a_directory() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let storage = FileJwksSecretStorage::new(directory.path().to_path_buf());

    let Err(RollerError::SecretLoad { source }) = storage.load() else {
        panic!("reading a directory as a secret file fails");
    };
    let backend = source
        .downcast_ref::<FileJwksSecretStorageError>()
        .expect("the load error carries a file storage source");

    assert!(matches!(backend, FileJwksSecretStorageError::Read { .. }));
}
