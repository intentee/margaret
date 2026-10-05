use std::ptr;

use serde::de::DeserializeOwned;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwt_verification::bearer_token_profile::BearerTokenProfile;
use margaret_jwt_verification::verified_jwt::VerifiedJwt;
use margaret_trusted_issuer::issuer_verification::IssuerVerification;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::attributed_bearer_token::AttributedBearerToken;

pub enum RoutedBearerToken<'request, 'trusted> {
    Absent,
    Attributed(AttributedBearerToken<'request, 'trusted>),
}

impl RoutedBearerToken<'_, '_> {
    pub async fn admit<TClaims: DeserializeOwned, TProfile: BearerTokenProfile>(
        &self,
        trusted_issuer: &TrustedIssuer,
    ) -> TokenAdmission<VerifiedJwt<TClaims, TProfile>> {
        let Self::Attributed(AttributedBearerToken {
            jwt,
            presented_at,
            trusted_issuer: addressee,
        }) = self
        else {
            return TokenAdmission::Unaddressed;
        };

        if !ptr::eq(*addressee, trusted_issuer) {
            return TokenAdmission::Unaddressed;
        }

        match trusted_issuer
            .verify(
                jwt,
                &trusted_issuer.trust.token_trust().audience,
                *presented_at,
            )
            .await
        {
            IssuerVerification::KeysAwaited => {
                TokenAdmission::Refused(ResponseContinuation::from(Response::text(
                    503,
                    "The signing keys of the token issuer are not available yet",
                )))
            }
            IssuerVerification::Rejected(_) => TokenAdmission::Refused(ResponseContinuation::from(
                BearerChallenge::InvalidToken.response(),
            )),
            IssuerVerification::Verified(verified) => TokenAdmission::Admitted(verified),
        }
    }
}
