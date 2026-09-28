use std::sync::Arc;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_rotates_the_current_secret_when_the_holder_is_seeded() {
    let storage = MemoryJwksSecretStorage;
    let seeded = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let seeded_kid = seeded.current().kid().clone();
    let promoted_kid = seeded.next().kid().clone();
    let holder = JwksSecretHolder::default();

    holder.set(Some(Arc::new(seeded)));

    roll(&storage, &holder, Curve::P256).expect("the roll rotates the secret");

    let rotated = holder.get().expect("the holder is seeded");

    assert!(rotated.previous().is_retired_key(&seeded_kid));
    assert_eq!(rotated.current().kid(), &promoted_kid);
    assert_ne!(rotated.next().kid(), &promoted_kid);
}
