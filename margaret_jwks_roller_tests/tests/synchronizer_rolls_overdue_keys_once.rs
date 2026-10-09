use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;

const INTERVALS_OF_DOWNTIME: u32 = 5;

#[tokio::test]
async fn synchronizer_rolls_overdue_keys_once() {
    let stored = fresh_secret(SigningCurve::P256);
    let storage = Arc::new(FixtureSigningKeys::storing(&stored));
    let now = stored
        .rolled_at()
        .after(JWKS_ROLL_INTERVAL * INTERVALS_OF_DOWNTIME);
    let rolled = fixture_synchronizer(storage.clone())
        .synchronized(&HeldSecret::Unheld, now)
        .await
        .expect("the overdue keys roll");

    assert_eq!(rolled.generation(), SigningKeysGeneration::new(2));
    assert_eq!(rolled.rolled_at(), now);
    assert_eq!(rolled.current().kid(), stored.next().kid());
    assert_eq!(storage.accepted_writes().await, 1);
}
