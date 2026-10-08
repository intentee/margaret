use uuid::Uuid;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer::access_token_minting::AccessTokenMinting;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn issues_a_refresh_token_that_mints_access_tokens() {
    let store = rolled_store(fresh_secret(SigningCurve::P256)).await;
    let issued = store.issue_refresh_token(Uuid::from_u128(7), unix_time(1_000));

    assert!(matches!(
        store.mint_access_token(&issued.signed_claims, unix_time(1_000)),
        AccessTokenMinting::Minted(_)
    ));
}
