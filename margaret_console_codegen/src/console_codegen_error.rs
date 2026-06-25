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

    #[error("console command '{command}' has no #[constructor] method")]
    MissingCommandConstructor { command: String },

    #[error(
        "console argument '{parameter}' of console command '{command}' has a boolean type but no name; a flag requires a name"
    )]
    NamelessFlag { command: String, parameter: String },
}
