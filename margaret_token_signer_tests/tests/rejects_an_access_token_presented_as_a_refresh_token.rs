use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jws_verification::header_type::HeaderType;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::type_rejection::TypeRejection;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer::minted_tokens::MintedTokens;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_an_access_token_presented_as_a_refresh_token() {
    let secret = fresh_p256_secret();
    let refresh_token = sign_refresh_token(secret.current(), &refresh_claims(), 10_000);
    let AccessTokenMinting::Minted(MintedTokens { access_token, .. }) =
        mint_access_token(&secret, &refresh_token, unix_time(1_000))
    else {
        panic!("a valid refresh token mints an access token");
    };

    assert!(matches!(
        mint_access_token(&secret, &access_token, unix_time(1_000)),
        AccessTokenMinting::RejectedRefreshToken(JwtRejection::Type(TypeRejection::Mismatch {
            expected: JwtType::Jwt,
            found: HeaderType::Supported(JwtType::AccessToken),
        }))
    ));
}
