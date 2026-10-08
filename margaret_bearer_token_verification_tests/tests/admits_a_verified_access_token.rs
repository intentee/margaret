use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test]
async fn admits_a_verified_access_token() {
    let secret = fresh_secret(SigningCurve::P256);
    let trusted_issuer = held_trusted_issuer(fixture_trust(), secret.published_key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());

    let TokenAdmission::Admitted(verified) =
        admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await
    else {
        panic!("the token is admitted");
    };

    assert_eq!(verified.claims.sub, "subject");
}
