use uuid::Uuid;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwks_secret_store_tests::unix_time::unix_time;

#[test]
fn issues_session_access_tokens_for_the_access_token_lifetime() {
    let signed = rolled_store(fresh_secret(SigningCurve::P256)).issue_session_access_token(
        &SessionAccessTokenClaims {
            auth_time: unix_time(400),
            sid: Uuid::from_u128(7),
            sub: Uuid::from_u128(9),
        },
        "browser",
        unix_time(500),
    );

    assert_eq!(signed.exp, 500 + i64::from(ACCESS_TOKEN_LIFETIME_SECS));
}
