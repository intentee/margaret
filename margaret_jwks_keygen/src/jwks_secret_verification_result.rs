use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum JwksSecretVerificationResult<TClaims, TProfile> {
    Rejected(JwtRejection),
    SignedWithCurrent(VerifiedJwt<TClaims, TProfile>),
    SignedWithNextKey,
    SignedWithPrevious(VerifiedJwt<TClaims, TProfile>),
}
