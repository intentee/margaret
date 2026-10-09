use std::sync::Arc;

use margaret::framework::active_record::model::Model;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_key_set_name::SIGNING_KEY_SET_NAME;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_keys_vanished_from_the_store() {
    let started = started_with_signing_keys().await;
    let held = Arc::new(fresh_secret(SigningCurve::P256));

    seeded_signing_keys(&started.database, &held).await;
    SigningKeySet::query()
        .name
        .eq(SIGNING_KEY_SET_NAME.to_string())
        .delete(started.database.as_ref())
        .await
        .expect("the stored keys vanish");

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Held(held.clone()), held.rolled_at())
            .await,
        Err(RollerError::StoredKeysVanished { generation }) if generation == SigningKeysGeneration::FIRST
    ));
}
