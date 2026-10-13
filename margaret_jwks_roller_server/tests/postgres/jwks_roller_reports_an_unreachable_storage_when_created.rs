use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_signing_keys_tests::started_with_signing_keys::started_with_signing_keys;

#[tokio::test]
async fn jwks_roller_reports_an_unreachable_storage_when_created() {
    let started = started_with_signing_keys().await;
    let held_connection = started
        .database
        .connection()
        .await
        .expect("the idle connection of the pool is checked out");

    started.administration.make_unreachable().await;

    assert!(matches!(
        JwksRoller::create(
            Arc::clone(&started.database),
            Arc::new(FixtureRsaSigningKeys::default())
        )
        .await,
        Err(JwksRollerServerError::SecretRoll(
            RollerError::SecretLoad { .. }
        ))
    ));

    drop(held_connection);
}
