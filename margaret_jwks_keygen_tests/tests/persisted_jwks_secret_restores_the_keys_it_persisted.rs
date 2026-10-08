use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::persisted_document::persisted_document;
use margaret_jwks_keygen_tests::restored_document::restored_document;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_keygen_tests::verified_token::verified_token;
use margaret_jwt_verification::jwt_verification::JwtVerification;

#[test]
fn persisted_jwks_secret_restores_the_keys_it_persisted() {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let fresh = fresh_secret(SigningCurve::P256);
    let retired_token = claims.signed_by(fresh.current());
    let rolled = rolled_secret(&fresh);
    let current_token = claims.signed_by(rolled.current());
    let restored = restored_document(persisted_document(&rolled)).expect("the document restores");

    assert_eq!(restored.rolled_at(), rolled.rolled_at());
    assert_eq!(
        restored.retired()[0].retired_at(),
        rolled.retired()[0].retired_at()
    );
    assert!(matches!(
        verified_token(restored.token_key_set(), &current_token),
        JwtVerification::Verified(_)
    ));
    assert!(matches!(
        verified_token(restored.token_key_set(), &retired_token),
        JwtVerification::Verified(_)
    ));
}
