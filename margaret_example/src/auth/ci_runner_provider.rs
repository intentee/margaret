use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::github_actions_claims::GithubActionsClaims;
use crate::auth::trusted_workflow::TrustedWorkflow;
use crate::models::ci_runner::CiRunner;

#[singleton]
#[infers_authenticated_user(user_model = CiRunner)]
pub struct CiRunnerProvider {
    trusted_workflow: Arc<TrustedWorkflow>,
}

impl CiRunnerProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(trusted_workflow: Arc<TrustedWorkflow>) -> anyhow::Result<Self> {
        Ok(Self { trusted_workflow })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_ci_runner(
        &self,
        #[bearer_token(issuer = github_actions)] token: Option<
            VerifiedJwt<GithubActionsClaims, IdTokenProfile>,
        >,
    ) -> anyhow::Result<AuthenticatedUserOutcome<CiRunner>> {
        let Some(verified) = token else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        if !self.trusted_workflow.admits(&verified.claims) {
            return Ok(AuthenticatedUserOutcome::Interrupted(
                ResponseContinuation::from(Response::forbidden()),
            ));
        }

        let GithubActionsClaims {
            repository,
            repository_id,
            ..
        } = verified.claims;

        Ok(AuthenticatedUserOutcome::Authenticated(CiRunner {
            repository,
            repository_id,
        }))
    }
}
