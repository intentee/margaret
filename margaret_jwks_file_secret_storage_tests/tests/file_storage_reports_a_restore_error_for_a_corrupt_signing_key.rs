use std::fs;

use serde_json::Value;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_a_restore_error_for_a_corrupt_signing_key() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    let storage = FileJwksSecretStorage::new(path.clone());

    storage
        .persist(&sample_secret())
        .expect("the secret can be persisted");

    let mut document: Value =
        serde_json::from_slice(&fs::read(&path).expect("the persisted file is readable"))
            .expect("the persisted file is json");

    document["current"]["signing"]["pem"] = Value::String("not a pkcs8 pem".to_string());
    fs::write(&path, document.to_string()).expect("the corrupt fixture can be written");

    let source = storage
        .load()
        .err()
        .expect("a corrupt signing key fails to load");
    let backend = source
        .chain()
        .find_map(|error| error.downcast_ref::<FileJwksSecretStorageError>())
        .expect("the load error carries a file storage source");

    assert!(matches!(
        backend,
        FileJwksSecretStorageError::Restore { .. }
    ));
}
