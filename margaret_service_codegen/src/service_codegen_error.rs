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

    #[error(
        "#[process] parameter '{parameter}' of '{unit}' is unsupported; a service/ticker process method takes only &self and an optional CancellationToken"
    )]
    UnexpectedProcessParameter { unit: String, parameter: String },

    #[error("#[scheduled_with_tick_timer] '{ticker}' is missing the 'interval' argument")]
    TickerMissingInterval { ticker: String },
}
