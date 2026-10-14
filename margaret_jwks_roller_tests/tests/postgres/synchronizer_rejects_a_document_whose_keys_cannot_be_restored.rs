use std::sync::Arc;

use base64ct::Base64;
use base64ct::Encoding;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_jwks_roller_tests::stored_document::stored_document;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_rejects_a_document_whose_keys_cannot_be_restored() {
    let started = started_with_signing_keys().await;
    let mut document = persisted_document(&fresh_secret(SigningCurve::P256));

    document["ec"]["current"]["pkcs8"] = Base64::encode_string(b"not a pkcs8 document").into();
    stored_document(&started.database, document.to_string().as_bytes()).await;

    assert!(matches!(
        fixture_synchronizer(Arc::clone(&started.database))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::DocumentRestore { .. })
    ));
}
