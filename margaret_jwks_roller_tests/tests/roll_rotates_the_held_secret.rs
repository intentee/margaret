use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn roll_rotates_the_held_secret() {
    let storage = FixtureSigningKeys::empty();
    let seeded = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let seeded_kid = seeded.current().kid().clone();
    let promoted_kid = seeded.next().kid().clone();
    let holder = JwksSecretHolder::new(Arc::new(seeded));

    let rotated = roll(&storage, &holder, &FixtureRsaSigningKeys::default())
        .await
        .expect("the roll rotates the secret");

    assert_eq!(holder.get().current().kid(), rotated.current().kid());
    assert!(rotated.previous().is_retired_key(&seeded_kid));
    assert_eq!(rotated.current().kid(), &promoted_kid);
    assert_ne!(rotated.next().kid(), &promoted_kid);
    assert_eq!(storage.stored_documents().await, 1);
}
