use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum EnvironmentVariableError {
    #[error("the environment variable '{name}' is not set")]
    Missing { name: String },

    #[error("the environment variable '{name}' does not hold valid unicode")]
    NotUnicode { name: String },

    #[error("the environment variable '{name}' does not hold a valid '{value_type}': {source}")]
    Malformed {
        name: String,
        #[source]
        source: Box<dyn Error + Send + Sync>,
        value_type: &'static str,
    },
}
