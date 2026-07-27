use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::mint_access_token_outcome::MintAccessTokenOutcome;
use margaret_token_signer::refresh_token_rejection::RefreshTokenRejection;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_an_expired_refresh_token() {
    let secret = fresh_p256_secret();
    let refresh_token = sign_refresh_token(&secret.current.signing, &refresh_claims(100)).await;

    let result = mint_access_token(&secret, &refresh_token, unix_time(200)).await;

    assert!(matches!(
        result,
        Ok(MintAccessTokenOutcome::Rejected(
            RefreshTokenRejection::Expired
        ))
    ));
}
