use std::sync::Arc;

use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_an_unreachable_store() {
    let started = started_with_signing_keys().await;
    let held_connection = started
        .database
        .connection()
        .await
        .expect("the idle connection of the pool is checked out");

    started.administration.make_unreachable().await;

    let Err(error) = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
        .await
    else {
        panic!("the store cannot be read");
    };

    assert!(matches!(error, RollerError::SecretLoad { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not load its stored signing keys: the signing keys cannot be loaded from the database: the Select statement on table 'signing_key_sets' failed: the database is unavailable: Error occurred while creating a new object: db error"
    );

    drop(held_connection);
}
