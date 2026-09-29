use crate::auth::github_actions_claims::GithubActionsClaims;
use crate::auth::github_actions_event_name::GithubActionsEventName;
use crate::auth::github_actions_runner_environment::GithubActionsRunnerEnvironment;

pub struct TrustedWorkflow {
    pub git_ref: String,
    pub repository_id: String,
    pub repository_owner_id: String,
    pub workflow_ref: String,
}

impl TrustedWorkflow {
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
