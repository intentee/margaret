use async_trait::async_trait;
use url::Url;

use margaret::framework::jwks_endpoint::endpoint_error::EndpointError;
use margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_jwks_endpoint;
use margaret::framework::macros::singleton;
use margaret::framework::registered_claims::audience::Audience;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

use crate::auth::github_actions_issuer_identifier::GITHUB_ACTIONS_ISSUER_IDENTIFIER;
use crate::auth::github_actions_key_set_url::GITHUB_ACTIONS_KEY_SET_URL;

#[singleton]
#[provides_jwks_endpoint(github_actions)]
pub struct GithubActionsIssuer {
    token_trust: TokenTrust,
}

impl GithubActionsIssuer {
    /// # Errors
    ///
    /// Returns an error when the GitHub Actions issuer identifier is malformed.
    #[constructor]
    pub fn create(
        #[console_argument(from = "github-actions-audience")] audience: Audience,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            token_trust: TokenTrust {
                audience,
                issuer: GITHUB_ACTIONS_ISSUER_IDENTIFIER.parse()?,
            },
        })
    }
}

#[async_trait]
impl ProvidesEndpoint for GithubActionsIssuer {
    async fn provide(&self) -> anyhow::Result<Url> {
        Url::parse(GITHUB_ACTIONS_KEY_SET_URL).map_err(|source| {
            EndpointError::Resolution {
                source: Box::new(source),
            }
            .into()
        })
    }
}

impl DeclaresTokenTrust for GithubActionsIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
