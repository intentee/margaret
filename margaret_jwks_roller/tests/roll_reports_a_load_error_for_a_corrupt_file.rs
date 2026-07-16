use std::fs;

use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn roll_reports_a_load_error_for_a_corrupt_file() {
    let directory = tempdir().expect("a temporary directory");
    let path = directory.path().join("jwks.json");
    fs::write(&path, b"not json").expect("the corrupt file is written");
    let storage = FileJwksSecretStorage::new(path);
    let holder = JwksSecretHolder::default();

    assert!(matches!(
        roll(&storage, &holder, Curve::P256),
        Err(RollerError::Deserialize { .. })
    ));
}
