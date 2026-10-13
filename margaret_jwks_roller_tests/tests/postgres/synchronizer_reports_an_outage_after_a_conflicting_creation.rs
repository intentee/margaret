use margaret::framework::database::isolation::Isolation;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::held_signing_keys_creation::held_signing_keys_creation;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_reports_an_outage_after_a_conflicting_creation() {
    let started = started_with_signing_keys().await;
    let mut peer = started
        .database
        .connection()
        .await
        .expect("the peer connection is checked out");
    let creating = peer
        .transaction(Isolation::ReadCommitted)
        .await
        .expect("the peer transaction begins");

    held_signing_keys_creation(&creating, &fresh_secret(SigningCurve::P256)).await;

    let synchronizer = fixture_synchronizer(started.separate_pool().await);
    let synchronizing = tokio::spawn(async move {
        synchronizer
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await
    });

    started.administration.await_lock_waiters(1).await;
    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "signing_key_sets",
        )
        .await;
    creating.commit().await.expect("the peer creates the keys");

    assert!(matches!(
        synchronizing.await.expect("the synchronization joins"),
        Err(RollerError::SecretLoad { .. })
    ));
}
