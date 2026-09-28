use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum JwksSecretVerificationResult<TClaims> {
    Rejected(JwtRejection),
    SignedWithCurrent(VerifiedJwt<TClaims>),
    SignedWithNextKey,
    SignedWithPrevious(VerifiedJwt<TClaims>),
}
