use serde_json::Value;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn reports_a_malformed_access_token_as_a_verification_outcome() {
    assert!(matches!(
        rolled_store(fresh_secret(SigningCurve::P256))
            .await
            .verify_access_token::<Value>("not-a-valid-jwt", unix_time(500)),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::NotCompactJws))
    ));
}
