use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_seeds_a_fresh_secret_when_storage_is_absent() {
    let storage = MemoryJwksSecretStorage;
    let holder = JwksSecretHolder::default();

    roll(&storage, &holder, Curve::P256).expect("the roll seeds a fresh secret");

    let seeded = holder.get().expect("the holder is seeded");

    assert_eq!(seeded.previous.public.kid, seeded.current.public.kid);
    assert_ne!(seeded.next.public.kid, seeded.current.public.kid);
}
