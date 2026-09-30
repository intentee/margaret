use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;

#[test]
fn public_jwks_handler_serves_the_document_after_a_roll() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(MemoryJwksSecretStorage),
    });

    bundle
        .roll_and_publish()
        .expect("the first roll publishes the document");

    assert_eq!(bundle.public_jwks_handler().respond().status(), 200);
}
