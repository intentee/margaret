use margaret_identity_session::access_token_claims::AccessTokenClaims;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn mints_from_a_current_key_refresh_token() {
    let secret = fresh_p256_secret();
    let refresh = refresh_claims(10_000);
    let refresh_token = sign_refresh_token(secret.current(), &refresh);

    let AccessTokenMinting::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints an access token");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(access) =
        secret.verify_any::<AccessTokenClaims>(&access_token)
    else {
        panic!("the minted access token verifies");
    };

    assert_eq!(access.sub, refresh.sub);
    assert_eq!(access.iat, 1_000);
}
