use std::sync::Arc;

use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_reports_an_unreachable_store() {
    let storage = Arc::new(FixtureSigningKeys::empty());

    storage.break_down().await;

    let Err(error) = fixture_synchronizer(storage)
        .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
        .await
    else {
        panic!("the store cannot be read");
    };

    assert!(matches!(error, RollerError::SecretLoad { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not load its stored signing keys: the jwks secret backend is unreachable"
    );
}
