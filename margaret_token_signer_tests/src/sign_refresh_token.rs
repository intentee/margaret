use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;
use margaret_jwks_keygen::jwk_pair::JwkPair;

#[must_use]
pub fn sign_refresh_token(pair: &JwkPair, claims: &RefreshTokenClaims) -> String {
    pair.sign_json(&claims.to_json())
}
