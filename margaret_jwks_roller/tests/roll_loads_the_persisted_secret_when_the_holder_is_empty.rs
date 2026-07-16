use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_loads_the_persisted_secret_when_the_holder_is_empty() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));
    let persisted = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    storage.persist(&persisted).expect("persist succeeds");
    let holder = JwksSecretHolder::default();

    roll(&storage, &holder, Curve::P256).expect("the roll loads the persisted secret");

    let loaded = holder.get().expect("the holder is seeded");
    assert_eq!(loaded.current.public.kid, persisted.current.public.kid);
}
