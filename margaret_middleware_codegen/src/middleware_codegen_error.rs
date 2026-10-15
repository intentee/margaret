use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
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

    #[error(
        "#[handles_middleware_attribute] is only supported on structs, but '{target}' is not a struct"
    )]
    MiddlewareHandlerNotOnStruct { target: String },

    #[error(
        "#[middleware] applies only to a #[responds_to_http] or #[websocket_session] struct, but it is attached to the method '{method}'"
    )]
    MiddlewareOnMethod { method: String },

    #[error(
        "#[middleware] applies only to a #[responds_to_http] or #[websocket_session] struct, but '{target}' is neither"
    )]
    MiddlewareOutsideRoute { target: String },

    #[error(
        "the #[process] method of middleware '{middleware}' must declare exactly one type parameter, the forward target it passes along, or none"
    )]
    MiddlewareProcessGenerics { middleware: String },

    #[error(
        "the #[process] method of middleware '{middleware}' names no forward target: declare a type parameter for Next<TTarget>, or take Next<ServerForwardTarget> of one server"
    )]
    MiddlewareTargetWithoutNext { middleware: String },

    #[error(
        "the Next parameter of middleware '{middleware}' takes no single type argument naming its forward target"
    )]
    MiddlewareNextWithoutTypeArgument { middleware: String },

    #[error(
        "the Next parameter of middleware '{middleware}' names the forward target '{target}', which does not resolve to a type"
    )]
    MiddlewareTargetUnresolved { middleware: String, target: String },

    #[error("{site} attaches the middleware handled by '{handler}', which has no middleware plan")]
    UnplannedMiddlewareHandler { handler: String, site: String },
}
