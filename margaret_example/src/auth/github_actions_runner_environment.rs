use serde::Deserialize;

#[derive(Deserialize)]
pub enum GithubActionsRunnerEnvironment {
    #[serde(rename = "github-hosted")]
    GithubHosted,
    #[serde(rename = "self-hosted")]
    SelfHosted,
}
