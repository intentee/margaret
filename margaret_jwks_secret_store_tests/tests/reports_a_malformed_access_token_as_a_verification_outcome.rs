use serde_json::Value;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn reports_a_malformed_access_token_as_a_verification_outcome() {
    assert!(matches!(
        rolled_store(fresh_p256_secret())
            .verify_access_token::<Value>("not-a-valid-jwt", unix_time(500)),
        Ok(JwksSecretVerificationResult::Rejected(JwtRejection::Jws(
            JwsRejection::NotCompactJws
        )))
    ));
}
