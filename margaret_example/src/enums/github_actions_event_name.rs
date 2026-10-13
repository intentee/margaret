use serde::Deserialize;

#[derive(Deserialize)]
pub enum GithubActionsEventName {
    #[serde(rename = "push")]
    Push,
    #[serde(other)]
    OtherEvent,
}
