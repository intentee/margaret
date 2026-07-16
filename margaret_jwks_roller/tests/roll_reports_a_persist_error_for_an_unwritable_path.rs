use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;

#[test]
fn roll_reports_a_persist_error_for_an_unwritable_path() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(
        directory
            .path()
            .join("missing-subdirectory")
            .join("jwks.json"),
    );
    let holder = JwksSecretHolder::default();

    assert!(matches!(
        roll(&storage, &holder, Curve::P256),
        Err(RollerError::Write { .. })
    ));
}
