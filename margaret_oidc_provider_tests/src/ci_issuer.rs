use std::sync::Arc;

use chrono::Utc;
use serde_json::Value;
use serde_json::json;

use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::ci_exchanger::CiExchanger;

const CI_AUDIENCE: &str = "https://localhost";

fn publish_keys(key_set: &IssuerKeySet, secret: &JwksSecret) {
    key_set.hold(Arc::new(secret.key_set().clone()));
}

pub struct CiIssuer {
    pub exchanger: Arc<SubjectTokenExchanger>,
    pub secret: JwksSecret,
}

impl CiIssuer {
    #[must_use]
    pub fn awaiting_keys() -> Self {
        Self::with_trusted_issuer(CI_AUDIENCE, CiExchanger, |_key_set, _secret| {})
    }

    #[must_use]
    pub fn exchanging_with<TExchanger: ExchangesSubjectTokens>(exchanger: TExchanger) -> Self {
        Self::with_trusted_issuer(CI_AUDIENCE, exchanger, publish_keys)
    }

    #[must_use]
    pub fn publishing_keys() -> Self {
        Self::publishing_keys_for(CI_AUDIENCE)
    }

    #[must_use]
    pub fn publishing_keys_for(audience: &'static str) -> Self {
        Self::with_trusted_issuer(audience, CiExchanger, publish_keys)
    }

    fn with_trusted_issuer<TExchanger: ExchangesSubjectTokens>(
        audience: &'static str,
        exchanger: TExchanger,
        publish: impl FnOnce(&IssuerKeySet, &JwksSecret),
    ) -> Self {
        let secret = fresh_p256_secret();
        let key_set = Arc::new(IssuerKeySet::awaiting());

        publish(&key_set, &secret);

        let trusted_issuer = TrustedIssuer::polled(
            key_set,
            TokenTrust {
                audience,
                issuer: "https://ci.localhost",
            },
        );

        Self {
            exchanger: Arc::new(SubjectTokenExchanger::create(
                Arc::new(trusted_issuer),
                Arc::new(exchanger),
            )),
            secret,
        }
    }

    #[must_use]
    pub fn token(&self, repository: &str) -> String {
        self.token_addressed_to(&json!(CI_AUDIENCE), repository)
    }

    #[must_use]
    pub fn token_addressed_to(&self, audience: &Value, repository: &str) -> String {
        let now = Utc::now().timestamp();

        self.secret.current().sign_json(
            &json!({
                "aud": audience,
                "exp": now + 300,
                "iat": now,
                "iss": "https://ci.localhost",
                "repository": repository,
                "sub": "repo:intentee/margaret:ref:refs/heads/main",
            }),
            JwtType::Jwt,
        )
    }
}
