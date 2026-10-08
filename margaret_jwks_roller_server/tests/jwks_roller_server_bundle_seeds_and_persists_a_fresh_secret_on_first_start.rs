use std::sync::Arc;

use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;

#[tokio::test]
async fn jwks_roller_server_bundle_seeds_and_persists_a_fresh_secret_on_first_start() {
    let storage = Arc::new(FixtureSigningKeys::empty());

    let first_start = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: storage.clone(),
    })
    .await
    .expect("the first start seeds a fresh secret");

    assert!(matches!(
        first_start.jwks_secret_holder().get().previous(),
        PreviousKey::Absent
    ));
    assert_eq!(storage.stored_documents().await, 1);
}
