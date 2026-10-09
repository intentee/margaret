use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jws_verification::parameter_value::ParameterValue;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_registered_claims::numeric_date::NumericDate;

pub enum ClientKeySet {
    Own(Arc<JwksSecretStore>),
    Published {
        key_set: Arc<IssuerKeySet>,
        signing: JwsAlgorithm,
    },
}

impl ClientKeySet {
    #[must_use]
    pub fn own(store: Arc<JwksSecretStore>) -> Self {
        Self::Own(store)
    }

    #[must_use]
    pub fn published(key_set: Arc<IssuerKeySet>, signing: JwsAlgorithm) -> Self {
        Self::Published { key_set, signing }
    }

    pub(crate) async fn verify<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        now: NumericDate,
    ) -> IssuerVerification<TClaims, TProfile> {
        match self {
            Self::Own(store) => IssuerVerification::from(store.verify_own_jwt(jwt, now)),
            Self::Published { key_set, signing } => match jwt.algorithm() {
                ParameterValue::Supported(algorithm) if algorithm == signing => {
                    key_set.verify_refetching_rotated_keys(jwt, now).await
                }
                ParameterValue::Supported(_) | ParameterValue::Unsupported(_) => {
                    IssuerVerification::Rejected(JwtRejection::AlgorithmNotPinned {
                        pinned: *signing,
                    })
                }
            },
        }
    }
}
