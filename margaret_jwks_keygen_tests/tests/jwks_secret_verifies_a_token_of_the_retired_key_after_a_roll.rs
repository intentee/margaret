use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn jwks_secret_verifies_a_token_of_the_retired_key_after_a_roll() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = fresh_secret(SigningCurve::P256);
    let token = claims.signed_by(secret.current());

    assert!(matches!(
        verified_token(rolled_secret(&secret).token_key_set(), &token),
        JwtVerification::Verified(verified) if verified.claims == claims
    ));
}
