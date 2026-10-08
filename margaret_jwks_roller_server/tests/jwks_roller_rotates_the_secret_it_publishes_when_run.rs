use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_rotates_the_secret_it_publishes_when_run() {
    let roller = JwksRoller::create(
        Arc::new(FixtureSigningKeys::empty()),
        Arc::new(FixtureRsaSigningKeys::default()),
    )
    .await
    .expect("the first secret is rolled and published");
    let initial = roller.jwks_secret_holder().get();

    roller.run().await.expect("the secret rotates");

    let rotated = roller.jwks_secret_holder().get();

    assert!(rotated.previous().is_retired_key(initial.current().kid()));
    assert_eq!(roller.public_jwks_handler().respond().status(), 200);
}
