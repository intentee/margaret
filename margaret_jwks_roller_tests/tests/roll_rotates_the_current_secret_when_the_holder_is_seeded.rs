use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::roll::roll;

#[test]
fn roll_rotates_the_current_secret_when_the_holder_is_seeded() {
    let storage = MemoryJwksSecretStorage;
    let seeded = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let seeded_kid = seeded.current().kid().clone();
    let promoted_kid = seeded.next().kid().clone();
    let holder = JwksSecretHolder::default();

    holder.set(Some(Arc::new(seeded)));

    roll(
        &storage,
        &holder,
        SigningCurve::P256,
        &FixtureRsaSigningKeys::default(),
    )
    .expect("the roll rotates the secret");

    let rotated = holder.get().expect("the holder is seeded");

    assert!(rotated.previous().is_retired_key(&seeded_kid));
    assert_eq!(rotated.current().kid(), &promoted_kid);
    assert_ne!(rotated.next().kid(), &promoted_kid);
}
