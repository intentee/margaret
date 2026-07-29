use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::token_signer_error::TokenSignerError;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn reports_a_corrupt_key_when_verifying_a_refresh_token() {
    let mut secret = JwksSecret::fresh(Curve::P256).expect("the secret generates");
    let refresh_token = sign_refresh_token(&secret.current.signing, &refresh_claims(10_000)).await;

    secret.current.public.x = "invalid @@@".to_string();

    let result = mint_access_token(&secret, &refresh_token, unix_time(1_000)).await;

    assert!(matches!(
        result,
        Err(TokenSignerError::RefreshTokenVerification { .. })
    ));
}
