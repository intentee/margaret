use std::sync::Arc;

use zeroize::Zeroizing;

use margaret::framework::active_record::secret_text::SecretText;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::stored_document::stored_document;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_rejects_a_malformed_document() {
    let started = started_with_signing_keys().await;

    stored_document(
        &started.database,
        SecretText::new(Zeroizing::new("not a signing keys document".to_string())),
    )
    .await;

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::DocumentMalformed { .. })
    ));
}
