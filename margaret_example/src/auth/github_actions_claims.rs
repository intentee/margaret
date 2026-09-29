use serde::Deserialize;

use crate::auth::github_actions_event_name::GithubActionsEventName;
use crate::auth::github_actions_runner_environment::GithubActionsRunnerEnvironment;

#[derive(Deserialize)]
pub struct GithubActionsClaims {
    pub event_name: GithubActionsEventName,
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub repository: String,
    pub repository_id: String,
    pub repository_owner_id: String,
    pub runner_environment: GithubActionsRunnerEnvironment,
    pub workflow_ref: String,
}
