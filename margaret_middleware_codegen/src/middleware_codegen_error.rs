use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_injection_codegen::injection_error::InjectionError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;

#[derive(Debug, Error)]
pub enum MiddlewareCodegenError {
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
    Binding {
        #[from]
        source: RequestBindingError,
    },

    #[error(
        "#[handles_middleware_attribute] is only supported on structs, but '{target}' is not a struct"
    )]
    MiddlewareHandlerNotOnStruct { target: String },

    #[error("middleware '{middleware}' is missing the 'attribute' argument")]
    MissingMiddlewareHandles { middleware: String },

    #[error("{site} has a #[middleware(...)] attribute that must name exactly one middleware tag")]
    MalformedMiddleware { site: String },

    #[error(
        "{site} attaches the middleware tag '{tag}', but no #[handles_middleware_attribute] handles it"
    )]
    UnknownMiddleware { site: String, tag: String },

    #[error(
        "{site} attaches the middleware tag '{tag}', which is handled by more than one #[handles_middleware_attribute]"
    )]
    AmbiguousMiddleware { site: String, tag: String },
}
