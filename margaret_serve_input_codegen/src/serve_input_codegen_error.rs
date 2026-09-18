use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_environment_variable_codegen::environment_variable_codegen_error::EnvironmentVariableCodegenError;
use margaret_input_weaving::constructor_parameter::ConstructorParameter;

#[derive(Debug, Error)]
pub enum ServeInputCodegenError {
    #[error(
        "{site} declares both #[{first}] and #[{second}]; a constructor parameter is fed by exactly one serve input"
    )]
    AmbiguousServeInput {
        first: &'static str,
        second: &'static str,
        site: ConstructorParameter,
    },

    #[error(transparent)]
    ConsoleArgument {
        #[from]
        source: ConsoleArgumentCodegenError,
    },

    #[error(
        "the serve input '{name}' declared by '{owner}' differs from the one declared by '{first_owner}'; a shared serve input must be declared identically everywhere"
    )]
    ConflictingServeInput {
        first_owner: String,
        name: String,
        owner: String,
    },

    #[error(transparent)]
    EnvironmentVariable {
        #[from]
        source: EnvironmentVariableCodegenError,
    },

    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(
        "the positional console argument '{name}' is declared by both '{owner}' and '{first_owner}'; a positional argument belongs to exactly one console command"
    )]
    SharedPositionalConsoleArgument {
        first_owner: String,
        name: String,
        owner: String,
    },

    #[error(
        "{site} carries arguments on #[spiffe_http_client]; the injected client takes no arguments"
    )]
    SpiffeHttpClientTakesNoArguments { site: ConstructorParameter },

    #[error(
        "{site} carries arguments on #[spiffe_websocket_client]; the injected client takes no arguments"
    )]
    SpiffeWebSocketClientTakesNoArguments { site: ConstructorParameter },
}
