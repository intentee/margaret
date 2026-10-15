use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn persisted_jwks_secret_restores_signing_keys_whose_tokens_the_original_keys_verify() {
    let secret = fresh_secret(SigningCurve::P256);
    let restored = restored_document(persisted_document(&secret)).expect("the document restores");

    assert!(matches!(
        verified_token(
            secret.token_key_set(),
            &TestClaims {
                sub: "subject".to_string(),
            }
            .signed_by(restored.current())
        ),
        JwtVerification::Verified(_)
    ));
}
