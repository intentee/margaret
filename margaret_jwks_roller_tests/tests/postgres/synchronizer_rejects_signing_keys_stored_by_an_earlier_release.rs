use std::sync::Arc;

use serde_json::json;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::stored_document::stored_document;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_rejects_signing_keys_stored_by_an_earlier_release() {
    let started = started_with_signing_keys().await;
    let current = persisted_document(&fresh_secret(SigningCurve::P256));
    let earlier_release = json!({
        "current": current["ec"]["current"],
        "next": current["ec"]["next"],
        "previous": current["ec"]["current"],
    });

    stored_document(&started.database, earlier_release.to_string().as_bytes()).await;

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::DocumentOpen {
            source: SigningKeysError::DocumentMalformed(_)
        })
    ));
}
