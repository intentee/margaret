use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::refused_without_headers::refused_without_headers;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_while_the_signing_keys_are_unavailable() {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let trusted_issuer = TrustedIssuer::create(Arc::new(IssuerKeySet::awaiting()), fixture_trust());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());

    assert!(refused_without_headers(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        503
    ));
}
