use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::initial_secret::initial_secret;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;

#[test]
fn initial_secret_seeds_a_fresh_secret_when_storage_is_absent() {
    let seeded = initial_secret(
        &MemoryJwksSecretStorage,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("a fresh secret is seeded");

    assert!(matches!(seeded.previous(), PreviousKey::Absent));
    assert_ne!(seeded.next().kid(), seeded.current().kid());
}
