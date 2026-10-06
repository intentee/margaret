use std::collections::BTreeSet;
use std::sync::Arc;

use async_trait::async_trait;

use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::constructor;
use margaret::framework::macros::exchanges_subject_tokens;
use margaret::framework::macros::singleton;
use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret::framework::subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

use crate::auth::ci_publisher::CI_PUBLISHER;
use crate::auth::github_actions_claims::GithubActionsClaims;
use crate::auth::trusted_workflow::TrustedWorkflow;

#[singleton]
#[exchanges_subject_tokens(issuer = github_actions)]
pub struct GithubActionsExchanger {
    trusted_workflow: Arc<TrustedWorkflow>,
}

impl GithubActionsExchanger {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(trusted_workflow: Arc<TrustedWorkflow>) -> anyhow::Result<Self> {
        Ok(Self { trusted_workflow })
    }
}

#[async_trait]
impl ExchangesSubjectTokens for GithubActionsExchanger {
    type Claims = GithubActionsClaims;
    type Profile = IdTokenProfile;

    async fn exchange(
        &self,
        token: &VerifiedJwt<GithubActionsClaims, IdTokenProfile>,
    ) -> anyhow::Result<SubjectTokenExchange> {
        Ok(if self.trusted_workflow.admits(&token.claims) {
            SubjectTokenExchange::Granted {
                scopes: BTreeSet::new(),
                subject: CI_PUBLISHER,
            }
        } else {
            SubjectTokenExchange::Refused
        })
    }
}
