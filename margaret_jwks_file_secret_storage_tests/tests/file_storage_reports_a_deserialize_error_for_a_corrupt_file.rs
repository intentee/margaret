use std::fs;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_a_deserialize_error_for_a_corrupt_file() {
    let directory = tempfile::tempdir().expect("a temporary directory can be created");
    let path = directory.path().join("jwks.json");
    fs::write(&path, b"not json").expect("the corrupt fixture can be written");
    let storage = FileJwksSecretStorage::new(path);

    let source = storage
        .load(&FixtureRsaSigningKeys::default())
        .err()
        .expect("a corrupt secret file fails to load");
    let backend = source
        .chain()
        .find_map(|error| error.downcast_ref::<FileJwksSecretStorageError>())
        .expect("the load error carries a file storage source");

    assert!(matches!(
        backend,
        FileJwksSecretStorageError::Deserialize { .. }
    ));
}
