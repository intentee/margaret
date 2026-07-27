use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::token_signer_error::TokenSignerError;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn reports_a_verification_failure_when_the_current_public_key_is_corrupt() {
    let mut secret = fresh_p256_secret();
    let refresh_token = sign_refresh_token(&secret.current.signing, &refresh_claims(1_000)).await;

    secret.current.public.x = "not-base64url".to_string();

    assert!(matches!(
        mint_access_token(&secret, &refresh_token, unix_time(500)).await,
        Err(TokenSignerError::Verification { .. })
    ));
}
