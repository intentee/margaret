use std::path::PathBuf;

use margaret_jwks_file_secret_storage::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_file_secret_storage::file_jwks_secret_storage_error::FileJwksSecretStorageError;
use margaret_jwks_file_secret_storage_tests::sample_secret::sample_secret;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_an_error_when_the_path_is_a_filesystem_root() {
    let storage = FileJwksSecretStorage::new(PathBuf::from("/"));

    let source = storage
        .persist(&sample_secret())
        .expect_err("a filesystem root is not a usable secret file path");
    let backend = source
        .chain()
        .find_map(|error| error.downcast_ref::<FileJwksSecretStorageError>())
        .expect("the persist error carries a file storage source");

    assert!(matches!(
        backend,
        FileJwksSecretStorageError::PathHasNoParentDirectory { .. }
    ));
}
