use anyhow::Result;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::mint_access_token_outcome::MintAccessTokenOutcome;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn mints_from_a_current_key_refresh_token() -> Result<()> {
    let secret = fresh_p256_secret();
    let refresh = refresh_claims(10_000);
    let refresh_token = sign_refresh_token(&secret.current.signing, &refresh).await;

    let MintAccessTokenOutcome::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &refresh_token, unix_time(1_000)).await?
    else {
        anyhow::bail!("a valid refresh token was rejected");
    };

    let access: AccessTokenClaims = secret.current.public.verify(&access_token)?.must()?;

    assert_eq!(access.sub, refresh.sub);
    assert_eq!(access.iat, 1_000);

    Ok(())
}
