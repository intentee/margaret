use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::loaded_secret::LoadedSecret;

#[test]
fn file_storage_reloads_a_persisted_secret() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));
    let original = JwksSecret::fresh(Curve::P256).expect("a fresh secret");

    storage.persist(&original).expect("persist succeeds");

    assert!(matches!(storage.load(), Ok(LoadedSecret::Present(_))));
}
