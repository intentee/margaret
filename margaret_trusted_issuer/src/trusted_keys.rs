use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_registered_claims::numeric_date::NumericDate;

pub enum TrustedKeys {
    Own(Arc<JwksSecretStore>),
    Polled(Arc<IssuerKeySet>),
}

impl TrustedKeys {
    pub async fn verify<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        now: NumericDate,
    ) -> IssuerVerification<TClaims, TProfile> {
        match self {
            Self::Own(store) => IssuerVerification::from(store.verify_issued_jwt(jwt, now)),
            Self::Polled(key_set) => key_set.verify_refetching_rotated_keys(jwt, now).await,
        }
    }
}
