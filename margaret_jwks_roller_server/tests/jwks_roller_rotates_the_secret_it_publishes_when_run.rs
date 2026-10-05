use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;

#[test]
fn jwks_roller_rotates_the_secret_it_publishes_when_run() {
    let roller = JwksRoller::create(
        Arc::new(MemoryJwksSecretStorage),
        Arc::new(FixtureRsaSigningKeys::default()),
    )
    .expect("the first secret is rolled and published");
    let initial = roller.jwks_secret_holder().get();

    roller.run().expect("the secret rotates");

    let rotated = roller.jwks_secret_holder().get();

    assert!(rotated.previous().is_retired_key(initial.current().kid()));
    assert_eq!(roller.public_jwks_handler().respond().status(), 200);
}
