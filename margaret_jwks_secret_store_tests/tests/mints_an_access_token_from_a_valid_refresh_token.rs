use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer_tests::fixture_issuance::fixture_issuance;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn mints_an_access_token_from_a_valid_refresh_token() {
    let secret = fresh_secret(SigningCurve::P256);
    let refresh_token = sign_refresh_token(
        secret.current(),
        &fixture_issuance(),
        &refresh_claims(),
        1_000,
    );

    assert!(matches!(
        rolled_store(secret)
            .await
            .mint_access_token(&refresh_token, unix_time(500)),
        AccessTokenMinting::Minted(_)
    ));
}
