use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_injection_codegen::injection_error::InjectionError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;

#[derive(Debug, Error)]
pub enum ConsoleCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error(transparent)]
    Container {
        #[from]
        source: ContainerError,
    },

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
    ServeInput {
        #[from]
        source: ServeInputCodegenError,
    },

    #[error("#[console_command] is only supported on structs, but '{target}' is not a struct")]
    ConsoleCommandNotOnStruct { target: String },

    #[error(
        "the #[process] runner of console command '{command}' takes parameter '{parameter}'; a runner may only take &self and an optional CancellationToken, console arguments belong on the #[constructor]"
    )]
    ConsoleCommandRunnerArgument { command: String, parameter: String },

    #[error(
        "parameter '{parameter}' of the #[process] runner of console command '{command}' carries #[{marker}], which is only available in an HTTP responder, an HTTP middleware, a WebSocket session builder, or an #[infer_from_request] method"
    )]
    ConsoleCommandRunnerRequestBinding {
        command: String,
        marker: String,
        parameter: String,
    },

    #[error("console command '{command}' is missing the 'name' argument")]
    MissingCommandName { command: String },

    #[error(
        "console command '{command}' injects the #[spiffe_http_client], which is only provisioned while serving; a console command cannot use the SPIFFE HTTP client"
    )]
    ConsoleCommandInjectsSpiffeHttpClient { command: String },

    #[error(
        "command '{command}' registers the console command name '{name}', which is already registered by command '{existing_command}'"
    )]
    DuplicateCommandName {
        command: String,
        existing_command: String,
        name: String,
    },
}
