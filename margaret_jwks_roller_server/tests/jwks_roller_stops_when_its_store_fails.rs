use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::roller_error::RollerError;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test(start_paused = true)]
async fn jwks_roller_stops_when_its_store_fails() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let roller = JwksRoller::create(storage.clone(), Arc::new(FixtureRsaSigningKeys::default()))
        .await
        .expect("the roller starts");

    storage.break_down().await;

    assert!(matches!(
        roller.run(CancellationToken::new()).await,
        Err(JwksRollerServerError::SecretRoll(
            RollerError::SecretLoad { .. }
        ))
    ));
}
