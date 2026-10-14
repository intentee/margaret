use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller::signing_keys_synchronizer::SigningKeysSynchronizer;
use margaret_jwks_roller_tests::seeded_signing_keys::seeded_signing_keys;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_signing_keys::signing_keys_error::SigningKeysError;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn synchronizer_rejects_signing_keys_sealed_under_another_key() {
    let started = started_with_signing_keys().await;

    seeded_signing_keys(&started.database, &fresh_secret(SigningCurve::P256)).await;

    assert!(matches!(
        SigningKeysSynchronizer {
            curve: SigningCurve::P256,
            database: Arc::clone(&started.database),
            encryption_key: Arc::new(
                "KqwDP+cFajlSzByKJhDwlRmmfnTx/8X2RhO2UsgAylY="
                    .parse()
                    .expect("the other key is an AES-256 key"),
            ),
            rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        }
        .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
        .await,
        Err(RollerError::DocumentOpen {
            source: SigningKeysError::DocumentOpen(_)
        })
    ));
}
