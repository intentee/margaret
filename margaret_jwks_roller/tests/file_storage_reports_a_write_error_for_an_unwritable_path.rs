use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_reports_a_write_error_for_an_unwritable_path() {
    let directory = tempdir().expect("a temporary directory");
    let path = directory
        .path()
        .join("missing-subdirectory")
        .join("jwks.json");
    let temporary_path = directory
        .path()
        .join("missing-subdirectory")
        .join("jwks.json.tmp");
    let storage = FileJwksSecretStorage::new(path.clone());
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");

    let error = storage
        .persist(&secret)
        .expect_err("writing to a missing directory fails");

    assert_eq!(
        error.to_string(),
        format!(
            "failed to write the jwks secret to '{}': {}",
            path.display(),
            std::fs::File::create(&temporary_path)
                .expect_err("creating in a missing directory fails")
        )
    );
}
