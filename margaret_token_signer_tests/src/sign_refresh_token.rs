use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_issuance::token_issuance::TokenIssuance;

#[must_use]
pub fn sign_refresh_token(
    pair: &JwkPair,
    TokenIssuance { audience, issuer }: &TokenIssuance,
    claims: &RefreshTokenClaims,
    exp: i64,
) -> String {
    pair.sign_json(
        &claims.to_payload(&RegisteredClaims {
            aud: AudienceClaim::Single(audience.as_str().to_string()),
            exp: NumericDate::new(exp),
            iat: NumericDate::new(0),
            iss: issuer.as_str().to_string(),
            nbf: None,
        }),
        JwtType::Jwt,
    )
}
