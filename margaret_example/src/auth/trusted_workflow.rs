use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::auth::github_actions_claims::GithubActionsClaims;
use crate::auth::github_actions_event_name::GithubActionsEventName;
use crate::auth::github_actions_runner_environment::GithubActionsRunnerEnvironment;

#[singleton]
pub struct TrustedWorkflow {
    pub git_ref: String,
    pub repository_id: String,
    pub repository_owner_id: String,
    pub workflow_ref: String,
}

impl TrustedWorkflow {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "github-actions-ref")] git_ref: String,
        #[console_argument(from = "github-actions-repository-id")] repository_id: String,
        #[console_argument(from = "github-actions-repository-owner-id")]
        repository_owner_id: String,
        #[console_argument(from = "github-actions-workflow-ref")] workflow_ref: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            git_ref,
            repository_id,
            repository_owner_id,
            workflow_ref,
        })
    }

    #[must_use]
    pub fn admits(
        &self,
        GithubActionsClaims {
            event_name,
            git_ref,
            repository: _,
            repository_id,
            repository_owner_id,
            runner_environment,
            workflow_ref,
        }: &GithubActionsClaims,
    ) -> bool {
        matches!(event_name, GithubActionsEventName::Push)
            && matches!(
                runner_environment,
                GithubActionsRunnerEnvironment::SelfHosted
            )
            && *git_ref == self.git_ref
            && *repository_id == self.repository_id
            && *repository_owner_id == self.repository_owner_id
            && *workflow_ref == self.workflow_ref
    }
}
