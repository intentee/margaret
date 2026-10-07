use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum IssuerVerification<TClaims, TProfile> {
    KeysAwaited,
    Rejected(JwtRejection),
    Verified(VerifiedJwt<TClaims, TProfile>),
}
