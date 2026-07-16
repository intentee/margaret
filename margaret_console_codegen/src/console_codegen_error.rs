use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_injection_codegen::injection_error::InjectionError;

#[derive(Debug, Error)]
pub enum ConsoleCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(transparent)]
    Injection {
        #[from]
        source: InjectionError,
    },

    #[error(transparent)]
    ConsoleArgument {
        #[from]
        source: ConsoleArgumentCodegenError,
    },

    #[error("#[console_command] is only supported on structs, but '{target}' is not a struct")]
    ConsoleCommandNotOnStruct { target: String },

    #[error("console command '{command}' is missing the 'name' argument")]
    MissingCommandName { command: String },

    #[error(
        "command '{command}' registers the console command name '{name}', which is already registered by command '{existing_command}'"
    )]
    DuplicateCommandName {
        command: String,
        existing_command: String,
        name: String,
    },
}
