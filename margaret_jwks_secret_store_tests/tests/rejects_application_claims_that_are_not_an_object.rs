use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_application_claims_that_are_not_an_object() {
    assert!(matches!(
        rolled_store(fresh_p256_secret()).sign_access_token(&"demo", unix_time(500)),
        Err(JwksSecretStoreError::AccessTokenClaims(
            ClaimsMergeError::NotAnObject
        ))
    ));
}
