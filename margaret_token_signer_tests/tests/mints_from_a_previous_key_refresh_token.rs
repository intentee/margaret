use anyhow::Result;

use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::verifies_token::VerifiesToken;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn mints_from_a_previous_key_refresh_token() -> Result<()> {
    let secret = fresh_p256_secret().rotate()?;
    let refresh = refresh_claims(10_000);
    let refresh_token = sign_refresh_token(&secret.previous.signing, &refresh).await;

    let AccessTokenMinting::Minted(MintedTokens {
        access_token,
        refresh_token,
    }) = mint_access_token(&secret, &refresh_token, unix_time(1_000)).await?
    else {
        panic!("a refresh token signed by the previous key mints an access token");
    };

    let access: AccessTokenClaims = secret
        .current
        .public
        .verify(&access_token)?
        .verified()
        .expect("the token verifies");
    let migrated: RefreshTokenClaims = secret
        .current
        .public
        .verify(&refresh_token)?
        .verified()
        .expect("the token verifies");

    assert_eq!(access.sub, refresh.sub);
    assert_eq!(migrated.jti, refresh.jti);

    Ok(())
}
