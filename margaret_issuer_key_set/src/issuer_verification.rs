use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;

pub enum IssuerVerification<TClaims, TProfile> {
    KeysAwaited,
    Rejected(JwtRejection),
    Verified(VerifiedJwt<TClaims, TProfile>),
}

impl<TClaims, TProfile> From<JwtVerification<TClaims, TProfile>>
    for IssuerVerification<TClaims, TProfile>
{
    fn from(verification: JwtVerification<TClaims, TProfile>) -> Self {
        match verification {
            JwtVerification::Rejected(rejection) => Self::Rejected(rejection),
            JwtVerification::Verified(verified) => Self::Verified(verified),
        }
    }
}
