use std::sync::Arc;

use chrono::Utc;
use serde_json::json;

use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_subject_token_exchange::subject_token_exchanger::SubjectTokenExchanger;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::ci_exchanger::CiExchanger;

pub struct CiIssuer {
    pub exchanger: Arc<SubjectTokenExchanger>,
    pub secret: JwksSecret,
}

impl CiIssuer {
    #[must_use]
    pub fn awaiting_keys() -> Self {
        Self::with_trusted_issuer(|_trusted_issuer, _secret| {})
    }

    #[must_use]
    pub fn publishing_keys() -> Self {
        Self::with_trusted_issuer(|trusted_issuer, secret| {
            trusted_issuer
                .key_set
                .hold(Arc::new(secret.key_set().clone()));
        })
    }

    fn with_trusted_issuer(publish: impl FnOnce(&TrustedIssuer, &JwksSecret)) -> Self {
        let secret = fresh_p256_secret();
        let trusted_issuer = TrustedIssuer::for_oidc_issuer(
            Arc::new(IssuerMetadata::awaiting()),
            Arc::new(TokenTrustDeclaration {
                trust: TokenTrust {
                    audience: "https://localhost"
                        .parse()
                        .expect("the audience is not empty"),
                    issuer: "https://ci.localhost"
                        .parse()
                        .expect("the ci issuer is an https url"),
                },
            }),
        );

        publish(&trusted_issuer, &secret);

        Self {
            exchanger: Arc::new(SubjectTokenExchanger::create(
                Arc::new(trusted_issuer),
                Arc::new(CiExchanger),
            )),
            secret,
        }
    }

    #[must_use]
    pub fn token(&self, repository: &str) -> String {
        let now = Utc::now().timestamp();

        self.secret.current().sign_json(
            &json!({
                "aud": "https://localhost",
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
