use serde_json::json;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn reports_the_secret_is_unavailable_when_signing_before_a_roll() {
    assert!(matches!(
        unrolled_store().sign_access_token(&json!({ "name": "demo" }), unix_time(0)),
        Err(JwksSecretStoreError::SecretUnavailable)
    ));
}
