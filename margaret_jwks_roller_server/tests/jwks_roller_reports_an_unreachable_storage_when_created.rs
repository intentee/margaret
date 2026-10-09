use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_reports_an_unreachable_storage_when_created() {
    let storage = Arc::new(FixtureSigningKeys::empty());

    storage.break_down().await;

    assert!(matches!(
        JwksRoller::create(storage, Arc::new(FixtureRsaSigningKeys::default())).await,
        Err(JwksRollerServerError::SecretRoll(
            RollerError::SecretLoad { .. }
        ))
    ));
}
