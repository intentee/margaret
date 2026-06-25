use margaret_attributes::attribute_error::AttributeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsoleCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("#[console_command] is only supported on structs, but '{target}' is not a struct")]
    ConsoleCommandNotOnStruct { target: String },

    #[error("console command '{command}' is missing the 'name' argument")]
    MissingCommandName { command: String },

    #[error("console command '{command}' has no #[runner] method")]
    MissingCommandRunner { command: String },

    #[error(
        "parameter '{parameter}' of console command '{command}' is not marked #[console_argument]; every runner parameter must be a console argument"
    )]
    UnmarkedRunnerParameter { command: String, parameter: String },

    #[error(
        "console argument '{parameter}' of console command '{command}' has a boolean type but no name; a flag requires a name"
    )]
    NamelessFlag { command: String, parameter: String },
}
