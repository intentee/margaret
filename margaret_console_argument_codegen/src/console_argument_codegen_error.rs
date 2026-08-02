use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum ConsoleArgumentCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

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

    #[error(
        "console argument '{parameter}' of '{owner}' has a generic value type '{value_type}'; a console argument value type must be a single concrete type, never a generic like Vec<T>"
    )]
    GenericValueType {
        owner: String,
        parameter: String,
        value_type: String,
    },

    #[error(
        "console argument '{parameter}' of '{owner}' has value type '{value_type}', which could not be resolved to a concrete type; import or fully qualify it"
    )]
    UnresolvableValueType {
        owner: String,
        parameter: String,
        value_type: String,
    },
}
