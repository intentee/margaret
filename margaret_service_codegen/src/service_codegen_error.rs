use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_injection_codegen::injection_error::InjectionError;

#[derive(Debug, Error)]
pub enum ServiceCodegenError {
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

    #[error(
        "the #[process] runner of '{path}' takes parameter '{parameter}'; a runner may only take &self and an optional CancellationToken, console arguments belong on the #[constructor]"
    )]
    RunnerArgument { path: String, parameter: String },

    #[error(
        "parameter '{parameter}' of the #[process] runner of '{path}' carries #[{marker}], which is only available in an HTTP responder, an HTTP middleware, a WebSocket session builder, or an #[infer_from_request] method"
    )]
    RunnerRequestBinding {
        marker: String,
        parameter: String,
        path: String,
    },

    #[error("#[service] is only supported on structs, but '{path}' is not a struct")]
    ServiceNotAStruct { path: String },

    #[error(
        "#[scheduled_with_tick_timer] is only supported on structs, but '{path}' is not a struct"
    )]
    TickerNotAStruct { path: String },

    #[error(
        "'{path}' carries more than one of #[console_command]/#[service]/#[scheduled_with_tick_timer]; these roles are mutually exclusive"
    )]
    ConflictingRoles { path: String },

    #[error("#[scheduled_with_tick_timer] '{ticker}' is missing the 'interval' argument")]
    TickerMissingInterval { ticker: String },
}
