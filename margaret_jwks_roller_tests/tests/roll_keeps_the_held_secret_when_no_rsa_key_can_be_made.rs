use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::malformed_rsa_signing_keys::MalformedRsaSigningKeys;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn roll_keeps_the_held_secret_when_no_rsa_key_can_be_made() {
    let storage = FixtureSigningKeys::empty();
    let seeded = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let seeded_kid = seeded.current().kid().clone();
    let holder = JwksSecretHolder::new(Arc::new(seeded));

    assert!(matches!(
        roll(&storage, &holder, &MalformedRsaSigningKeys).await,
        Err(RollerError::KeyGeneration(_))
    ));
    assert_eq!(holder.get().current().kid(), &seeded_kid);
    assert_eq!(storage.stored_documents().await, 0);
}
