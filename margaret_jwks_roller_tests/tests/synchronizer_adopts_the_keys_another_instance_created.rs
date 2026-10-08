use std::sync::Arc;

use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_adopts_the_keys_another_instance_created() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let created = fixture_synchronizer(storage.clone())
        .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
        .await
        .expect("fresh keys are created");
    let adopted = fixture_synchronizer(storage.clone())
        .synchronized(
            &HeldSecret::Unheld,
            NumericDate::new(0).after(JWKS_ROLL_INTERVAL / 2),
        )
        .await
        .expect("the created keys are adopted");

    assert_eq!(adopted.current().kid(), created.current().kid());
    assert_eq!(adopted.next().kid(), created.next().kid());
    assert_eq!(adopted.rolled_at(), created.rolled_at());
    assert_eq!(storage.accepted_writes().await, 1);
}
