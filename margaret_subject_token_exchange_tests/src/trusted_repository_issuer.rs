use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
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
            Arc::new(self.trusted_issuer(PROVIDER_AUDIENCE)),
            Arc::new(RepositoryExchanger::<TProfile> {
                profile: PhantomData,
            }),
        ))
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
        audience: &str,
    ) -> Arc<SubjectTokenExchanger> {
        let trusted_issuer = self.trusted_issuer(audience);

        trusted_issuer
            .key_set
            .hold(Arc::new(self.secret.key_set().clone()));

        Arc::new(SubjectTokenExchanger::create(
            Arc::new(trusted_issuer),
            Arc::new(RepositoryExchanger::<TProfile> {
                profile: PhantomData,
            }),
        ))
    }

    #[must_use]
    pub fn token(&self, repository: &str, jwt_type: JwtType) -> String {
        self.token_for(&json!(PROVIDER_AUDIENCE), repository, jwt_type)
    }

    #[must_use]
    pub fn token_for(&self, audience: &Value, repository: &str, jwt_type: JwtType) -> String {
        signed_by(&self.secret, self.issuer, audience, repository, jwt_type)
    }

    fn trusted_issuer(&self, audience: &str) -> TrustedIssuer {
        TrustedIssuer::for_oidc_issuer(
            Arc::new(IssuerMetadata::awaiting()),
            Arc::new(TokenTrustDeclaration {
                trust: TokenTrust {
                    audience: audience.parse().expect("the audience is not empty"),
                    issuer: self.issuer.parse().expect("the issuer is an https url"),
                },
            }),
        )
    }
}
