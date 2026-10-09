use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn jwks_secret_rejects_a_token_of_another_secret() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(fresh_secret(SigningCurve::P256).current());

    assert!(matches!(
        verified_token(fresh_secret(SigningCurve::P256).token_key_set(), &token),
        JwtVerification::Rejected(JwtRejection::Jws(JwsRejection::UnknownKeyId { .. }))
    ));
}
