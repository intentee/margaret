use crate::jwt_rejection::JwtRejection;
use crate::verified_jwt::VerifiedJwt;

pub enum JwtVerification<TClaims, TProfile> {
    Rejected(JwtRejection),
    Verified(VerifiedJwt<TClaims, TProfile>),
}
