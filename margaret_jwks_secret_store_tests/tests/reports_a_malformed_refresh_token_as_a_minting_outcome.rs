use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn reports_a_malformed_refresh_token_as_a_minting_outcome() {
    assert!(matches!(
        rolled_store(fresh_p256_secret())
            .await
            .mint_access_token("not-a-valid-jwt", unix_time(500)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Jws(JwsRejection::NotCompactJws))
    ));
}
