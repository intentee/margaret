use std::sync::Arc;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::unreachable_signing_keys::UnreachableSigningKeys;

#[tokio::test]
async fn roll_reports_a_persist_error_when_the_backend_is_unreachable() {
    let seeded = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let seeded_kid = seeded.current().kid().clone();
    let holder = JwksSecretHolder::new(Arc::new(seeded));

    let Err(error) = roll(
        &UnreachableSigningKeys,
        &holder,
        &FixtureRsaSigningKeys::default(),
    )
    .await
    else {
        panic!("the backend cannot be written");
    };

    assert!(matches!(error, RollerError::SecretPersist { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not store the rolled signing keys: the jwks secret backend is unreachable"
    );
    assert_eq!(holder.get().current().kid(), &seeded_kid);
}
