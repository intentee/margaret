use serde_json::json;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;

#[test]
fn reports_the_secret_is_unavailable_when_asserting_before_a_roll() {
    assert!(matches!(
        unrolled_store().sign_client_assertion(&json!({ "sub": "client" })),
        Err(JwksSecretStoreError::SecretUnavailable)
    ));
}
