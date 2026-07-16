use std::fs;
use std::os::unix::fs::PermissionsExt;

use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

#[test]
fn file_storage_tightens_permissions_on_a_preexisting_file() {
    let directory = tempdir().expect("a temporary directory");
    let path = directory.path().join("jwks.json");

    fs::write(&path, b"stale").expect("a preexisting file is written");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
        .expect("loose permissions are set");

    let storage = FileJwksSecretStorage::new(path.clone());
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");

    storage.persist(&secret).expect("persist succeeds");

    let mode = fs::metadata(&path)
        .expect("the persisted file exists")
        .permissions()
        .mode();

    assert_eq!(mode & 0o777, 0o600);
}
