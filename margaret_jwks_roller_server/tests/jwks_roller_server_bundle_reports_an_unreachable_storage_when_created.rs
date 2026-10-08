use std::sync::Arc;

use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_tests::unreachable_signing_keys::UnreachableSigningKeys;

#[tokio::test]
async fn jwks_roller_server_bundle_reports_an_unreachable_storage_when_created() {
    assert!(matches!(
        JwksRollerServerBundle::new(JwksRollerServerBundleParams {
            rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
            storage: Arc::new(UnreachableSigningKeys),
        })
        .await,
        Err(JwksRollerServerError::SecretRoll(_))
    ));
}
