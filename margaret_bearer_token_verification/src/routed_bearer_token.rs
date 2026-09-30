use std::ptr;

use serde::de::DeserializeOwned;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::response::Response;
use margaret_jwt_verification::bearer_token_profile::BearerTokenProfile;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_trusted_issuer::issuer_verification::IssuerVerification;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::attributed_bearer_token::AttributedBearerToken;
use crate::bearer_token_admission::BearerTokenAdmission;
use crate::refusal::refusal;

pub enum RoutedBearerToken<'request, 'trusted> {
    Absent,
    Attributed(AttributedBearerToken<'request, 'trusted>),
}

impl RoutedBearerToken<'_, '_> {
    pub async fn admit<TClaims: DeserializeOwned, TProfile: BearerTokenProfile>(
        &self,
        trusted_issuer: &TrustedIssuer,
    ) -> BearerTokenAdmission<TClaims, TProfile> {
        let Self::Attributed(AttributedBearerToken {
            jwt,
            presented_at,
            trusted_issuer: addressee,
        }) = self
        else {
            return BearerTokenAdmission::Unaddressed;
        };

        if !ptr::eq(*addressee, trusted_issuer) {
            return BearerTokenAdmission::Unaddressed;
        }

        let JwtProfiling::Profiled(profiled) = jwt.profile::<TProfile>() else {
            return refusal(BearerChallenge::InvalidToken.response());
        };

        match trusted_issuer
            .verify(
                &profiled,
                &trusted_issuer.trust.token_trust().audience,
                *presented_at,
            )
            .await
        {
            IssuerVerification::KeysAwaited => refusal(Response::text(
                503,
                "The signing keys of the token issuer are not available yet",
            )),
            IssuerVerification::Rejected(_) => refusal(BearerChallenge::InvalidToken.response()),
            IssuerVerification::Verified(verified) => BearerTokenAdmission::Admitted(verified),
        }
    }
}
