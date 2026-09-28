use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn preserves_the_registered_claims_of_the_refresh_token() {
    let secret = fresh_p256_secret();
    let original = RegisteredClaims {
        exp: NumericDate::new(10_000),
        iat: NumericDate::new(0),
        nbf: Some(NumericDate::new(500)),
    };
    let refresh_token = secret
        .current()
        .sign_json(&refresh_claims().to_payload(&original));

    let AccessTokenMinting::Minted(MintedTokens { refresh_token, .. }) =
        mint_access_token(&secret, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints new tokens");
    };
    let JwksSecretVerificationResult::SignedWithCurrent(reissued) =
        secret.verify_jwt::<RefreshTokenClaims>(&refresh_token, NumericDate::new(1_000))
    else {
        panic!("the reissued refresh token verifies");
    };

    assert_eq!(reissued.registered, original);
}
