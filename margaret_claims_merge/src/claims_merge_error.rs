use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClaimsMergeError {
    #[error("the application claims collide with the member '{member}'")]
    Colliding { member: String },

    #[error("the application claims are not a json object")]
    NotAnObject,

    #[error("the application claims could not be serialized to json: {0}")]
    Serialization(#[source] serde_json::Error),
}
