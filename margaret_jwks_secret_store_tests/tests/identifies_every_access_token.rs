use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

fn signed_token_identifier(store: &JwksSecretStore) -> String {
    let signed = store
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");
    let JwksSecretVerificationResult::SignedWithCurrent(verified) =
        store.verify_access_token::<Map<String, Value>>(&signed.signed_claims, unix_time(500))
    else {
        panic!("the signed token carries a token identifier");
    };

    verified
        .registered
        .jti
        .expect("the signed token carries a token identifier")
}

#[tokio::test]
async fn identifies_every_access_token() {
    let store = rolled_store(fresh_p256_secret()).await;

    assert_ne!(
        signed_token_identifier(&store),
        signed_token_identifier(&store)
    );
}
