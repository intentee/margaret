use std::sync::Arc;

use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn synchronizer_adopts_the_keys_another_instance_created() {
    let started = started_with_signing_keys().await;
    let created = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
        .await
        .expect("fresh keys are created");
    let adopted = fixture_synchronizer(started.separate_pool().await)
        .synchronized(
            &HeldSecret::Unheld,
            NumericDate::new(0).after(JWKS_ROLL_INTERVAL / 2),
        )
        .await
        .expect("the created keys are adopted");

    assert_eq!(adopted.current().kid(), created.current().kid());
    assert_eq!(adopted.next().kid(), created.next().kid());
    assert_eq!(adopted.rolled_at(), created.rolled_at());
    assert_eq!(
        StoredRevision::loaded(&started.database).await,
        StoredRevision::of(
            &SigningKeysRevision::from_secret(&created).expect("the keys serialize")
        )
    );
}
