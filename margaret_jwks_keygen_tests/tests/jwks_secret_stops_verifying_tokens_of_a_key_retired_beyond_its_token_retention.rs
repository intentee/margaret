use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn jwks_secret_stops_verifying_tokens_of_a_key_retired_beyond_its_token_retention() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = fresh_secret(SigningCurve::P256);
    let token = claims.signed_by(secret.current());
    let retained = rolled_secret(&rolled_secret(&secret));
    let lapsed = rolled_secret(&retained);

    assert!(matches!(
        verified_token(retained.token_key_set(), &token),
        JwtVerification::Verified(_)
    ));
    assert!(matches!(
        verified_token(lapsed.token_key_set(), &token),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
    assert!(matches!(
        verified_token(lapsed.published_key_set(), &token),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
}
