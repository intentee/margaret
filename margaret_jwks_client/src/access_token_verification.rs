use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum AccessTokenVerification<TClaims> {
    NotReady,
    Rejected(JwtRejection),
    Verified(VerifiedJwt<TClaims>),
}
