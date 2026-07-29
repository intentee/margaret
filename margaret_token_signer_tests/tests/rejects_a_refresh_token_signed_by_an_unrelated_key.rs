use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_a_refresh_token_signed_by_an_unrelated_key() {
    let secret = fresh_p256_secret();
    let stranger = fresh_p256_secret();
    let refresh_token =
        sign_refresh_token(&stranger.current.signing, &refresh_claims(10_000)).await;

    let result = mint_access_token(&secret, &refresh_token, unix_time(1_000)).await;

    assert!(matches!(
        result,
        Ok(AccessTokenMinting::UnknownRefreshTokenKey)
    ));
}
