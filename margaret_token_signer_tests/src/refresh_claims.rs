use uuid::Uuid;

use margaret_identity_session::refresh_token_claims::RefreshTokenClaims;

#[must_use]
pub fn refresh_claims() -> RefreshTokenClaims {
    RefreshTokenClaims {
        sub: Uuid::from_u128(2),
    }
}
