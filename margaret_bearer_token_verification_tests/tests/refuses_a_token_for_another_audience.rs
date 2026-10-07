use serde_json::json;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test]
async fn refuses_a_token_for_another_audience() {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let token = secret.current().sign_json(
        &json!({
            "aud": "someone-else",
            "exp": FAR_FUTURE_EXPIRY,
            "iat": 0,
            "iss": trust.issuer,
            "sub": "subject",
        }),
        JwtType::AccessToken,
    );
    let trusted_issuer = held_trusted_issuer(trust, secret.key_set().clone());

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
