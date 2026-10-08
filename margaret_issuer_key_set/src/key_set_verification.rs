use tokio::time::Instant;

use margaret_jwt_verification::jwt_rejection::JwtRejection;

use crate::issuer_verification::IssuerVerification;

pub(crate) enum KeySetVerification<TClaims, TProfile> {
    Settled(IssuerVerification<TClaims, TProfile>),
    KeysPossiblyRotated {
        fetched_at: Instant,
        rejection: JwtRejection,
    },
}

impl<TClaims, TProfile> KeySetVerification<TClaims, TProfile> {
    pub(crate) fn settled(self) -> IssuerVerification<TClaims, TProfile> {
        match self {
            Self::Settled(verification) => verification,
            Self::KeysPossiblyRotated { rejection, .. } => IssuerVerification::Rejected(rejection),
        }
    }
}
