use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer::mint_access_token::mint_access_token;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[test]
fn rejects_a_refresh_token_signed_with_the_next_key() {
    let issuance = fixture_issuance();
    let secret = fresh_p256_secret();
    let refresh_token = sign_refresh_token(secret.next(), &issuance, &refresh_claims(), 10_000);

    assert!(matches!(
        mint_access_token(&secret, &issuance, &refresh_token, unix_time(1_000)),
        AccessTokenMinting::RefreshTokenSignedWithNextKey
    ));
}
