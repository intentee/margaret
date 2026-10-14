use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::ci_runner::CiRunner;
use crate::auth::claims::github_actions_claims::GithubActionsClaims;
use crate::auth::policies::trusted_workflow::TrustedWorkflow;

#[singleton]
#[infers_authenticated_user(user_model = CiRunner)]
pub struct CiRunnerProvider {
    trusted_workflow: Arc<TrustedWorkflow>,
}

impl CiRunnerProvider {
    #[must_use]
    #[constructor]
    pub fn create(trusted_workflow: Arc<TrustedWorkflow>) -> Self {
        Self { trusted_workflow }
    }

    #[must_use]
    #[infer_from_request]
    pub fn infer_ci_runner(
        &self,
        #[bearer_token(issuer = github_actions)] token: Option<
            VerifiedJwt<GithubActionsClaims, IdTokenProfile>,
        >,
    ) -> AuthenticatedUserOutcome<CiRunner> {
        let Some(verified) = token else {
            return AuthenticatedUserOutcome::Anonymous;
        };

        if !self.trusted_workflow.admits(&verified.claims) {
            return AuthenticatedUserOutcome::Interrupted(Response::forbidden());
        }

        let GithubActionsClaims {
            repository,
            repository_id,
            ..
        } = verified.claims;

        AuthenticatedUserOutcome::Authenticated(CiRunner {
            repository,
            repository_id,
        })
    }
}
