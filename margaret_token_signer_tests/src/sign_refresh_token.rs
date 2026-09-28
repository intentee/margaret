use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwk_pair::JwkPair;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[must_use]
pub fn sign_refresh_token(pair: &JwkPair, claims: &RefreshTokenClaims, exp: i64) -> String {
    pair.sign_json(&claims.to_payload(&RegisteredClaims {
        exp: NumericDate::new(exp),
        iat: NumericDate::new(0),
        nbf: None,
    }))
}
