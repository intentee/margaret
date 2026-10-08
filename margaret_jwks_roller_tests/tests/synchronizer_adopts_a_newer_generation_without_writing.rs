use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn synchronizer_adopts_a_newer_generation_without_writing() {
    let started = started_with_signing_keys().await;
    let held = Arc::new(fresh_secret(SigningCurve::P256));
    let newer = rolled_secret(&rolled_secret(&rolled_secret(&held)));

    seeded_signing_keys(&started.database, &newer).await;

    let before = StoredRevision::loaded(&started.database).await;
    let adopted = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(&HeldSecret::Held(held), newer.rolled_at())
        .await
        .expect("the newer keys are adopted");

    assert_eq!(adopted.generation(), newer.generation());
    assert_eq!(adopted.current().kid(), newer.current().kid());
    assert_eq!(StoredRevision::loaded(&started.database).await, before);
}
