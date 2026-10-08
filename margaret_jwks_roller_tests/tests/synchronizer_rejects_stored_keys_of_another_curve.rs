use std::sync::Arc;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_roller::held_secret::HeldSecret;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jwks_roller_tests::fixture_synchronizer::fixture_synchronizer;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn synchronizer_rejects_stored_keys_of_another_curve() {
    let storage = FixtureSigningKeys::storing(&fresh_secret(SigningCurve::P384));

    assert!(matches!(
        fixture_synchronizer(Arc::new(storage))
            .synchronized(&HeldSecret::Unheld, NumericDate::new(0))
            .await,
        Err(RollerError::CurveMismatch {
            pinned: SigningCurve::P256,
            stored: SigningCurve::P384,
        })
    ));
}
