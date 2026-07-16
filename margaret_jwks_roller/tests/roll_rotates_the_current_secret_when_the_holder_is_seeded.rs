use std::sync::Arc;

use tempfile::tempdir;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::file_jwks_secret_storage::FileJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_rotates_the_current_secret_when_the_holder_is_seeded() {
    let directory = tempdir().expect("a temporary directory");
    let storage = FileJwksSecretStorage::new(directory.path().join("jwks.json"));
    let seeded = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let seeded_kid = seeded.current.public.kid.clone();
    let holder = JwksSecretHolder::default();
    holder.set(Some(Arc::new(seeded)));

    roll(&storage, &holder, Curve::P256).expect("the roll rotates the secret");

    let rotated = holder.get().expect("the holder is seeded");
    assert_eq!(rotated.previous.public.kid, seeded_kid);
    assert_ne!(rotated.current.public.kid, seeded_kid);
}
