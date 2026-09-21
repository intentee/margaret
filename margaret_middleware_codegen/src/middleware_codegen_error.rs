use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_injection_codegen::injection_error::InjectionError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
use margaret_tag_codegen::tag_error::TagError;

#[derive(Debug, Error)]
pub enum MiddlewareCodegenError {
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
    Binding {
        #[from]
        source: RequestBindingError,
    },

    #[error(transparent)]
    Tag {
        #[from]
        source: TagError,
    },

    #[error("middleware '{middleware}' is missing the 'attribute' argument")]
    MissingMiddlewareHandles { middleware: String },

    #[error("middleware '{middleware}' names a tag that is not a single plain name")]
    MalformedMiddlewareTag { middleware: String },

    #[error(
        "{site} attaches the middleware tag '{tag}', but no #[handles_middleware_attribute] handles it"
    )]
    UnknownMiddleware { site: String, tag: String },
}
