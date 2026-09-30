use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_seeds_a_fresh_secret_when_storage_is_absent() {
    let storage = MemoryJwksSecretStorage;
    let holder = JwksSecretHolder::default();

    roll(
        &storage,
        &holder,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("the roll seeds a fresh secret");

    let seeded = holder.get().expect("the holder is seeded");

    assert!(matches!(seeded.previous(), PreviousKey::Absent));
    assert_ne!(seeded.next().kid(), seeded.current().kid());
}
