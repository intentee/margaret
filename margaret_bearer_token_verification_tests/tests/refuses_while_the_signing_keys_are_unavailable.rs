use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::refused_without_headers::refused_without_headers;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_while_the_signing_keys_are_unavailable() {
    let secret = fresh_secret(SigningCurve::P256);
    let trusted_issuer = TrustedIssuer::polled(Arc::new(IssuerKeySet::awaiting()), fixture_trust());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());

    assert!(refused_without_headers(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        503
    ));
}
