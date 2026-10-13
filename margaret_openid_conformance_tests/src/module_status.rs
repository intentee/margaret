use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModuleStatus {
    Configured,
    Created,
    Finished,
    Interrupted,
    Running,
    Waiting,
}
