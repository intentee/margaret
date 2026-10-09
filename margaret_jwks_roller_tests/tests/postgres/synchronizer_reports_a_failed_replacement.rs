use std::sync::Arc;

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;
use margaret_signing_keys_tests::stored_revision::StoredRevision;

#[tokio::test]
async fn synchronizer_reports_a_failed_replacement() {
    let started = started_with_signing_keys().await;
    let stored = fresh_secret(SigningCurve::P256);

    seeded_signing_keys(&started.database, &stored).await;

    let before = StoredRevision::loaded(&started.database).await;

    started
        .administration
        .revoke(
            TablePrivilege::Update,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;

    let Err(error) = fixture_synchronizer(Arc::clone(&started.database))
        .synchronized(
            &HeldSecret::Unheld,
            stored.rolled_at().after(JWKS_ROLL_INTERVAL),
        )
        .await
    else {
        panic!("the replacement cannot be stored");
    };

    assert!(matches!(error, RollerError::SecretReplace { .. }));
    assert_eq!(
        error.to_string(),
        "the application could not replace its stored signing keys: the signing keys cannot be replaced in the database: the Update statement on table 'signing_key_sets' failed: the database does not execute the statement: db error"
    );
    assert_eq!(StoredRevision::loaded(&started.database).await, before);
}
