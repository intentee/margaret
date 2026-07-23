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
        "console argument '{parameter}' of '{owner}' declares both `from` and `positional`; an argument is either named or positional, never both"
    )]
    NamedAndPositional { owner: String, parameter: String },

    #[error(
        "console argument '{parameter}' of '{owner}' declares neither `from = \"...\"` nor `positional`; it must be exactly one of them"
    )]
    NeitherNamedNorPositional { owner: String, parameter: String },

    #[error(
        "console argument '{parameter}' of '{owner}' is `positional` but its type is a boolean; a boolean is a named flag, never positional"
    )]
    BooleanPositional { owner: String, parameter: String },

    #[error(
        "console argument '{parameter}' of '{owner}' is `positional`, but '{owner}' is not a #[console_command]; positional arguments are only allowed on console commands"
    )]
    PositionalOutsideCommand { owner: String, parameter: String },

    #[error(
        "the command-line argument '{name}' declared by '{owner}' has a different type than the '{name}' declared by '{first_owner}'; a shared named argument must have the same type everywhere"
    )]
    ConflictingConsoleArgumentType {
        first_owner: String,
        name: String,
        owner: String,
    },

    #[error(
        "the command-line argument '{name}' is declared both as a positional and as a named argument within the same command; a name must resolve to exactly one argument"
    )]
    ConflictingConsoleArgumentId { name: String },
}
