use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_malformed_refresh_token() {
    assert!(matches!(
        mint_access_token(&fresh_p256_secret(), "not-a-valid-jwt", unix_time(1_000)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Jws(JwsRejection::NotCompactJws))
    ));
}
