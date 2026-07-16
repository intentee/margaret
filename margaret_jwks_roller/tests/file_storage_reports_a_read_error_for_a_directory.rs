use tempfile::tempdir;

use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_a_read_error_for_a_directory() {
    let directory = tempdir().expect("a temporary directory");
    let path = directory.path().to_path_buf();
    let storage = FileJwksSecretStorage::new(path.clone());

    let error = storage.load().err().expect("reading a directory fails");

    assert_eq!(
        error.to_string(),
        format!(
            "failed to read the jwks secret at '{}': {}",
            path.display(),
            std::fs::read(&path).expect_err("reading a directory fails")
        )
    );
}
