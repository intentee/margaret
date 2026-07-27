use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::mint_access_token_outcome::MintAccessTokenOutcome;
use margaret_token_signer::refresh_token_rejection::RefreshTokenRejection;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_a_malformed_refresh_token() {
    let secret = fresh_p256_secret();

    let result = mint_access_token(&secret, "not-a-valid-jwt", unix_time(1_000)).await;

    assert!(matches!(
        result,
        Ok(MintAccessTokenOutcome::Rejected(
            RefreshTokenRejection::Invalid
        ))
    ));
}
