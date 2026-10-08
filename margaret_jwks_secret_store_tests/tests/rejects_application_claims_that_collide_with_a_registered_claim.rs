use serde_json::json;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_application_claims_that_collide_with_a_registered_claim() {
    assert!(matches!(
        rolled_store(fresh_p256_secret()).await
            .sign_access_token(&json!({ "exp": 9_999, "name": "demo" }), unix_time(500)),
        Err(JwksSecretStoreError::AccessTokenClaims(ClaimsMergeError::Colliding { member })) if member == "exp"
    ));
}
