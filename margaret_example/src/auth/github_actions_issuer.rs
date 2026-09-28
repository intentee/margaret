use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::trusts_oidc_issuer;
use margaret::framework::registered_claims::audience::Audience;
use margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;
use margaret::framework::token_trust::token_trust::TokenTrust;

use crate::auth::github_actions_issuer_identifier::GITHUB_ACTIONS_ISSUER_IDENTIFIER;

#[singleton]
#[trusts_oidc_issuer(github_actions)]
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

impl DeclaresTokenTrust for GithubActionsIssuer {
    fn token_trust(&self) -> &TokenTrust {
        &self.token_trust
    }
}
