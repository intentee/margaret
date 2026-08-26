use std::error::Error;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
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
