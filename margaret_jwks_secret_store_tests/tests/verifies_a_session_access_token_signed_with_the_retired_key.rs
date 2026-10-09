use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwks_secret_store_tests::unix_time::unix_time;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn verifies_a_session_access_token_signed_with_the_retired_key() {
    let secret = fresh_secret(SigningCurve::P256);
    let signed = rolled_store(secret.clone()).issue_session_access_token(
        &SessionAccessTokenClaims {
            auth_time: unix_time(400),
            sid: Uuid::from_u128(7),
            sub: Uuid::from_u128(9),
        },
        "browser",
        unix_time(500),
    );

    assert!(matches!(
        rolled_store(rolled_secret(&secret)).verify_session_access_token(
            &signed.signed_claims,
            "browser",
            unix_time(500)
        ),
        JwtVerification::Verified(verified) if verified.kid.as_ref() == Some(secret.current().kid())
    ));
}
