use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_restores_the_stored_keys_across_a_restart() {
    let storage = Arc::new(FixtureSigningKeys::empty());
    let first_start =
        JwksRoller::create(storage.clone(), Arc::new(FixtureRsaSigningKeys::default()))
            .await
            .expect("the first start creates the keys")
            .jwks_secret_holder()
            .get();
    let restart = JwksRoller::create(storage.clone(), Arc::new(FixtureRsaSigningKeys::default()))
        .await
        .expect("the restart restores the keys")
        .jwks_secret_holder()
        .get();

    assert_eq!(restart.current().kid(), first_start.current().kid());
    assert_eq!(restart.next().kid(), first_start.next().kid());
    assert_eq!(storage.accepted_writes().await, 1);
}
