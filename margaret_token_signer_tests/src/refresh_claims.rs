use uuid::Uuid;

use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;

#[must_use]
pub fn refresh_claims(exp: i64) -> RefreshTokenClaims {
    RefreshTokenClaims {
        exp,
        iat: 0,
        jti: Uuid::from_u128(1),
        sub: Uuid::from_u128(2),
    }
}
