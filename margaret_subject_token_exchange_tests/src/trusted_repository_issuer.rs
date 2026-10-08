use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_subject_token_exchange::subject_token_profile::SubjectTokenProfile;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::provider_audience::PROVIDER_AUDIENCE;
use crate::repository_exchanger::RepositoryExchanger;
use crate::signed_by::signed_by;

pub struct TrustedRepositoryIssuer {
    pub issuer: &'static str,
    pub secret: JwksSecret,
}

impl TrustedRepositoryIssuer {
    #[must_use]
    pub fn named(issuer: &'static str) -> Self {
        Self {
            issuer,
            secret: fresh_p256_secret(),
        }
    }

    #[must_use]
    pub fn awaiting_keys<TProfile: SubjectTokenProfile + Send + Sync + 'static>(
        &self,
    ) -> Arc<SubjectTokenExchanger> {
        Arc::new(SubjectTokenExchanger::create(
            Arc::new(self.trusted_issuer(Arc::new(IssuerKeySet::awaiting()), PROVIDER_AUDIENCE)),
            Arc::new(RepositoryExchanger::<TProfile> {
                profile: PhantomData,
            }),
        ))
    }

    #[must_use]
    pub fn exchanging_with<TExchanger: ExchangesSubjectTokens>(
        &self,
        exchanger: TExchanger,
    ) -> Arc<SubjectTokenExchanger> {
        self.published_exchanger(PROVIDER_AUDIENCE, exchanger)
    }

    #[must_use]
    pub fn publishing_keys<TProfile: SubjectTokenProfile + Send + Sync + 'static>(
        &self,
    ) -> Arc<SubjectTokenExchanger> {
        self.publishing_keys_for::<TProfile>(PROVIDER_AUDIENCE)
    }

    #[must_use]
    pub fn publishing_keys_for<TProfile: SubjectTokenProfile + Send + Sync + 'static>(
        &self,
        audience: &'static str,
    ) -> Arc<SubjectTokenExchanger> {
        self.published_exchanger(
            audience,
            RepositoryExchanger::<TProfile> {
                profile: PhantomData,
            },
        )
    }

    #[must_use]
    pub fn token(&self, repository: &str, jwt_type: JwtType) -> String {
        self.token_for(&json!(PROVIDER_AUDIENCE), repository, jwt_type)
    }

    #[must_use]
    pub fn token_for(&self, audience: &Value, repository: &str, jwt_type: JwtType) -> String {
        signed_by(&self.secret, self.issuer, audience, repository, jwt_type)
    }

    fn published_exchanger<TExchanger: ExchangesSubjectTokens>(
        &self,
        audience: &'static str,
        exchanger: TExchanger,
    ) -> Arc<SubjectTokenExchanger> {
        let key_set = Arc::new(IssuerKeySet::awaiting());

        key_set.hold(Arc::new(self.secret.key_set().clone()));

        Arc::new(SubjectTokenExchanger::create(
            Arc::new(self.trusted_issuer(key_set, audience)),
            Arc::new(exchanger),
        ))
    }

    fn trusted_issuer(&self, key_set: Arc<IssuerKeySet>, audience: &'static str) -> TrustedIssuer {
        TrustedIssuer::polled(
            key_set,
            TokenTrust {
                audience,
                issuer: self.issuer,
            },
        )
    }
}
