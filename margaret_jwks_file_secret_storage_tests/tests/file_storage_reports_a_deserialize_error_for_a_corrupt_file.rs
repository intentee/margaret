use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn file_storage_reports_a_deserialize_error_for_a_corrupt_file() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    std::fs::write(&path, b"not json").expect("the corrupt fixture can be written");
    let storage = FileJwksSecretStorage::new(path);

    let Err(RollerError::SecretLoad { source }) = storage.load() else {
        panic!("a corrupt secret file fails to load");
    };
    let backend = source
        .downcast_ref::<FileJwksSecretStorageError>()
        .expect("the load error carries a file storage source");

    assert!(matches!(
        backend,
        FileJwksSecretStorageError::Deserialize { .. }
    ));
}
