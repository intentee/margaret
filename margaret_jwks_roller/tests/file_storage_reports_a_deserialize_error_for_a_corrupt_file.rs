use std::fs;

use tempfile::tempdir;

use margaret_jwks_key_gen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_a_deserialize_error_for_a_corrupt_file() {
    let directory = tempdir().expect("a temporary directory");
    let path = directory.path().join("jwks.json");

    fs::write(&path, b"not json").expect("the corrupt file is written");

    let storage = FileJwksSecretStorage::new(path.clone());

    let error = storage.load().err().expect("deserializing garbage fails");

    assert_eq!(
        error.to_string(),
        format!(
            "failed to deserialize the jwks secret at '{}': {}",
            path.display(),
            serde_json::from_slice::<PersistedJwksSecret>(b"not json")
                .err()
                .expect("deserializing garbage fails")
        )
    );
}
