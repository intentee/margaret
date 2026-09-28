use serde::Deserialize;
use serde_json::json;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_secret_store::access_token_signing::AccessTokenSigning;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[derive(Debug, Deserialize, PartialEq)]
struct DemoClaims {
    name: String,
}

#[test]
fn signs_access_token_claims_that_verify_with_the_current_key() {
    let store = rolled_store(fresh_p256_secret());
    let Ok(AccessTokenSigning::Signed(signed)) =
        store.sign_access_token(&json!({ "name": "demo" }), unix_time(500))
    else {
        panic!("the claims are signed");
    };
    let Ok(JwksSecretVerificationResult::SignedWithCurrent(verified)) =
        store.verify_access_token::<DemoClaims>(&signed.signed_claims, unix_time(500))
    else {
        panic!("the signed claims verify with the current key");
    };

    assert_eq!(
        verified.claims,
        DemoClaims {
            name: "demo".to_string()
        }
    );
}
