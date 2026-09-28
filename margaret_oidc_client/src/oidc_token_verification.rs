use margaret_jwt_verification::verified_jwt::VerifiedJwt;

use crate::oidc_token_rejection::OidcTokenRejection;

pub enum OidcTokenVerification<TClaims> {
    Absent,
    NotBearer,
    Rejected(OidcTokenRejection),
    Unavailable,
    Verified(VerifiedJwt<TClaims>),
}
