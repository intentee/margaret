use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum BearerTokenVerification<TClaims> {
    Absent,
    MalformedAuthorization,
    NotBearer,
    Rejected(JwtRejection),
    Unavailable,
    Verified(VerifiedJwt<TClaims>),
}
