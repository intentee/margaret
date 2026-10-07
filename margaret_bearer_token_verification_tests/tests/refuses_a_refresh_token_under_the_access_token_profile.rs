use uuid::Uuid;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[tokio::test]
async fn refuses_a_refresh_token_under_the_access_token_profile() {
    let trust = fixture_trust();
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let refresh_token = secret.current().sign_json(
        &RefreshTokenClaims {
            sub: Uuid::from_u128(2),
        }
        .to_payload(&RegisteredClaims {
            aud: AudienceClaim::Single(trust.audience.to_string()),
            exp: NumericDate::new(FAR_FUTURE_EXPIRY),
            iat: Some(NumericDate::new(0)),
            iss: trust.issuer.to_string(),
            jti: Some(Uuid::from_u128(1).to_string()),
            nbf: None,
        }),
        JwtType::Refresh,
    );
    let trusted_issuer = held_trusted_issuer(trust, secret.key_set().clone());

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, &format!("Bearer {refresh_token}")).await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
