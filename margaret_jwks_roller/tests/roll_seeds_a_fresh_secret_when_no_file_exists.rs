use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_seeds_a_fresh_secret_when_no_file_exists() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));
    let holder = JwksSecretHolder::default();

    roll(&storage, &holder, Curve::P256).expect("the roll seeds a fresh secret");

    assert!(holder.get().is_some());
}
