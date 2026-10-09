use std::sync::Arc;

use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_key_set::SigningKeySet;
use margaret_signing_keys::signing_keys_revision::SigningKeysRevision;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

const PEER_GENERATION: u64 = 7;

#[tokio::test]
async fn synchronizer_reports_created_keys_it_cannot_observe() {
    let started = started_with_signing_keys().await;

    SigningKeySet::create(
        &started.database,
        &SigningKeysRevision {
            generation: SigningKeysGeneration::new(PEER_GENERATION),
            ..SigningKeysRevision::from_secret(&fresh_secret(SigningCurve::P256))
                .expect("the peer keys serialize")
        },
    )
    .await
    .expect("the peer creates its keys");
    started
        .administration
        .hide_rows(
            TableNamespace::Framework,
            "signing_key_sets",
            &format!("generation <> {PEER_GENERATION}"),
        )
        .await;

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::CreatedKeysNotObserved)
    ));
}
