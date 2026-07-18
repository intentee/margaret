use std::sync::Arc;

use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::roll::roll;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::unreachable_jwks_secret_storage::UnreachableJwksSecretStorage;

#[test]
fn roll_reports_a_persist_error_when_the_backend_is_unreachable() {
    let storage = UnreachableJwksSecretStorage;
    let seeded = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let seeded_kid = seeded.current.public.kid.clone();
    let holder = JwksSecretHolder::default();

    holder.set(Some(Arc::new(seeded)));

    let Err(error) = roll(&storage, &holder, Curve::P256) else {
        panic!("the backend cannot be written");
    };

    assert!(matches!(error, RollerError::SecretPersist { .. }));
    assert_eq!(
        error.to_string(),
        "failed to persist the rolled jwks secret: the jwks secret backend is unreachable"
    );

    let unchanged = holder.get().expect("the holder keeps its previous secret");

    assert_eq!(unchanged.current.public.kid, seeded_kid);
}
