use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

const INTERVALS_OF_DOWNTIME: u32 = 5;

#[tokio::test]
async fn synchronizer_rolls_overdue_keys_once() {
    let started = started_with_signing_keys().await;
    let stored = fresh_secret(SigningCurve::P256);

    seeded_signing_keys(&started.database, &stored).await;

    let now = stored
        .rolled_at()
        .after(JWKS_ROLL_INTERVAL * INTERVALS_OF_DOWNTIME);
    let rolled = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(&HeldSecret::Unheld, now)
        .await
        .expect("the overdue keys roll");

    assert_eq!(rolled.generation(), SigningKeysGeneration::new(2));
    assert_eq!(rolled.rolled_at(), now);
    assert_eq!(rolled.current().kid(), stored.next().kid());
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(&SigningKeysRevision::from_secret(&rolled).expect("the keys serialize"))
    );
}
