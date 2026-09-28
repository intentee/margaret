use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::unrolled_store::unrolled_store;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn reports_the_secret_is_unavailable_when_minting_before_a_roll() {
    assert!(matches!(
        unrolled_store().mint_access_token("token", unix_time(0)),
        Err(JwksSecretStoreError::SecretUnavailable)
    ));
}
