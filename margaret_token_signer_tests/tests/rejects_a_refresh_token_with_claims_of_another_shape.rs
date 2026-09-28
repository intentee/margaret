use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_refresh_token_with_claims_of_another_shape() {
    let secret = fresh_p256_secret();
    let access_claims = refresh_claims(10_000).mint_access_token_claims(unix_time(0));
    let token = secret.current().sign_json(&access_claims.to_json());

    assert!(matches!(
        mint_access_token(&secret, &token, unix_time(1_000)),
        AccessTokenMinting::MalformedRefreshTokenClaims(_)
    ));
}
