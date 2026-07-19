use std::sync::Arc;

use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;
use margaret_jwks_roller_server::jwks_roller_server_error::JwksRollerServerError;
use margaret_jwks_roller_tests::unreachable_jwks_secret_storage::UnreachableJwksSecretStorage;

#[test]
fn jwks_roll_service_reports_a_roll_failure() {
    let bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(UnreachableJwksSecretStorage),
    });

    let error = bundle
        .roll_and_publish()
        .expect_err("the backend cannot be reached");

    assert!(matches!(error, JwksRollerServerError::SecretRoll(_)));
}
