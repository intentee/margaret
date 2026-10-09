use uuid::Uuid;

use margaret_identity_session::session_access_token_claims::SessionAccessTokenClaims;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwks_secret_store_tests::unix_time::unix_time;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn rejects_a_session_access_token_presented_for_another_audience() {
    let store = rolled_store(fresh_secret(SigningCurve::P256));
    let signed = store.issue_session_access_token(
        &SessionAccessTokenClaims {
            auth_time: unix_time(400),
            sid: Uuid::from_u128(7),
            sub: Uuid::from_u128(9),
        },
        "browser",
        unix_time(500),
    );

    assert!(matches!(
        store.verify_session_access_token(&signed.signed_claims, "artifacts", unix_time(501)),
        JwtVerification::Rejected(JwtRejection::Claims(ClaimsRejection::AudienceMismatch { expected, .. }))
            if expected == "artifacts"
    ));
}
