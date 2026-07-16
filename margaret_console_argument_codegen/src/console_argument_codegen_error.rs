use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ConsoleArgumentCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(
        "parameter '{parameter}' of '{owner}' is not marked #[console_argument]; every process parameter must be a console argument"
    )]
    UnmarkedProcessParameter { owner: String, parameter: String },

    #[error(
        "console argument #{parameter} of '{owner}' is missing `from = \"...\"`; it must name the command-line argument it binds"
    )]
    MissingFrom { owner: String, parameter: String },

    #[error(
        "the command-line argument '{name}' declared by '{owner}' is already declared by '{existing_owner}'; every argument must be unique"
    )]
    DuplicateConsoleArgument {
        existing_owner: String,
        name: String,
        owner: String,
    },
}
