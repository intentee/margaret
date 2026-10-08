use std::sync::Arc;

use zeroize::Zeroizing;

use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_document::SigningKeysDocument;
use margaret_jwks_roller::signing_keys_revision::SigningKeysRevision;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_rejects_a_malformed_document() {
    let storage = FixtureSigningKeys::holding(SigningKeysRevision {
        document: SigningKeysDocument::new(Zeroizing::new(
            "not a signing keys document".to_string(),
        )),
        generation: SigningKeysGeneration::FIRST,
    });

    assert!(matches!(
        fixture_synchronizer(Arc::new(storage))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::DocumentMalformed { .. })
    ));
}
