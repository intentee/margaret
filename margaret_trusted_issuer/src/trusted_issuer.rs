use std::sync::Arc;

use serde::de::DeserializeOwned;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_issuer_key_set::issuer_verification::IssuerVerification;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwt_verification::attributed_jwt::AttributedJwt;
use margaret_jwt_verification::jwt_addressee::JwtAddressee;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_profile::JwtProfile;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_token_trust::token_trust::TokenTrust;

use crate::trusted_keys::TrustedKeys;

pub struct TrustedIssuer {
    keys: TrustedKeys,
    pub trust: TokenTrust,
}

impl TrustedIssuer {
    #[must_use]
    pub fn own(store: Arc<JwksSecretStore>, trust: TokenTrust) -> Self {
        Self {
            keys: TrustedKeys::Own(store),
            trust,
        }
    }

    #[must_use]
    pub fn polled(key_set: Arc<IssuerKeySet>, trust: TokenTrust) -> Self {
        Self {
            keys: TrustedKeys::Polled(key_set),
            trust,
        }
    }

    pub async fn verify<TClaims: DeserializeOwned, TProfile: JwtProfile>(
        &self,
        jwt: &AttributedJwt<'_>,
        now: NumericDate,
    ) -> IssuerVerification<TClaims, TProfile> {
        self.keys.verify(jwt, now).await
    }
}

impl JwtAddressee for TrustedIssuer {
    fn jwt_expectation(&self) -> JwtExpectation<'_> {
        self.trust.expectation()
    }
}
