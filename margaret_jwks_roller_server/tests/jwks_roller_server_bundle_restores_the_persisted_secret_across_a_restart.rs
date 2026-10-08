use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwks_roller_tests::fixture_signing_keys::FixtureSigningKeys;
use margaret_jws_verification::key_id::KeyId;

async fn started_kid(storage: Arc<FixtureSigningKeys>) -> KeyId {
    JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage,
    })
    .await
    .expect("the server starts with a secret")
    .jwks_secret_holder()
    .get()
    .current()
    .kid()
    .clone()
}

#[tokio::test]
async fn jwks_roller_server_bundle_restores_the_persisted_secret_across_a_restart() {
    let storage = Arc::new(FixtureSigningKeys::empty());

    assert_eq!(
        started_kid(storage.clone()).await,
        started_kid(storage).await
    );
}
