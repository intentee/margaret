use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_jwt_verification::jwt_addressee::JwtAddressee;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_profiling::JwtProfiling;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::issuer_verification::IssuerVerification;
use crate::key_set_locator::KeySetLocator;
use crate::key_set_verification::KeySetVerification;
use crate::verify_with_key_set::verify_with_key_set;

pub struct TrustedIssuer {
    pub key_set: IssuerKeySet,
    pub locator: KeySetLocator,
    pub trust: Arc<dyn DeclaresTokenTrust>,
}

impl TrustedIssuer {
    #[must_use]
    pub fn for_jwks_endpoint(
        endpoint: Arc<dyn ProvidesEndpoint>,
        trust: Arc<dyn DeclaresTokenTrust>,
    ) -> Self {
        Self {
            key_set: IssuerKeySet::awaiting(),
            locator: KeySetLocator::Endpoint(endpoint),
            trust,
        }
    }

    #[must_use]
    pub fn for_oidc_issuer(
        metadata: Arc<IssuerMetadata>,
        trust: Arc<dyn DeclaresTokenTrust>,
    ) -> Self {
        Self {
            key_set: IssuerKeySet::awaiting(),
            locator: KeySetLocator::Discovery {
                discovery_url: oidc_discovery_url(&trust.token_trust().issuer),
                metadata,
            },
            trust,
        }
    }

    pub async fn verify<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        audience: &Audience,
        now: NumericDate,
    ) -> IssuerVerification<TClaims, TProfile> {
        let jwt = match jwt.profile::<TProfile>() {
            JwtProfiling::Profiled(profiled) => profiled,
            JwtProfiling::Rejected(rejection) => {
                return IssuerVerification::Rejected(JwtRejection::Type(rejection));
            }
        };
        let snapshot = self.key_set.snapshot();

        match verify_with_key_set(&jwt, &snapshot.holding, audience, now) {
            KeySetVerification::UnknownKey {
                fetched_at,
                rejection,
            } if fetched_at.elapsed() >= ISSUER_FETCH_SPACING => {
                match self.key_set.refreshed_since(&snapshot).await {
                    KeySetRefresh::PollingStopped => IssuerVerification::Rejected(rejection),
                    KeySetRefresh::Refreshed(holding) => {
                        verify_with_key_set(&jwt, &holding, audience, now).settled()
                    }
                }
            }
            verification => verification.settled(),
        }
    }
}

impl JwtAddressee for TrustedIssuer {
    fn jwt_issuer(&self) -> &IssuerIdentifier {
        &self.trust.token_trust().issuer
    }
}
