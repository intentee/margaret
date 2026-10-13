use std::collections::BTreeSet;
use std::sync::Arc;

use async_trait::async_trait;

use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::constructor;
use margaret::framework::macros::exchanges_tokens_from;
use margaret::framework::macros::singleton;
use margaret::framework::subject_token_exchange::exchanges_subject_tokens::ExchangesSubjectTokens;
use margaret::framework::subject_token_exchange::subject_token_exchange::SubjectTokenExchange;

use crate::auth::claims::github_actions_claims::GithubActionsClaims;
use crate::auth::policies::trusted_workflow::TrustedWorkflow;
use crate::constants::CI_PUBLISHER;

#[singleton]
#[exchanges_tokens_from(issuer = github_actions)]
pub struct GithubActionsExchanger {
    trusted_workflow: Arc<TrustedWorkflow>,
}

impl GithubActionsExchanger {
    #[must_use]
    #[constructor]
    pub fn create(trusted_workflow: Arc<TrustedWorkflow>) -> Self {
        Self { trusted_workflow }
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
