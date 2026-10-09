use uuid::Uuid;

use margaret_identity_session::refresh_token_lifetime_secs::REFRESH_TOKEN_LIFETIME_SECS;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn issues_a_refresh_token_for_the_refresh_token_lifetime() {
    let issued = rolled_store(fresh_secret(SigningCurve::P256))
        .await
        .issue_refresh_token(Uuid::from_u128(7), unix_time(1_000));

    assert_eq!(issued.exp, 1_000 + i64::from(REFRESH_TOKEN_LIFETIME_SECS));
}
