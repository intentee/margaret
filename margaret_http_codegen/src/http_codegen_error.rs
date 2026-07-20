use matchit::InsertError;
use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_injection_codegen::injection_error::InjectionError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;

#[derive(Debug, Error)]
pub enum HttpCodegenError {
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

    #[error("#[responds_to_http] is only supported on structs, but '{target}' is not a struct")]
    RespondsToHttpNotOnStruct { target: String },

    #[error("responder '{responder}' is missing the 'method' argument")]
    MissingHttpMethod { responder: String },

    #[error("responder '{responder}' has an invalid HTTP method '{method}': {source}")]
    InvalidHttpMethod {
        responder: String,
        method: String,
        source: http::method::InvalidMethod,
    },

    #[error("responder '{responder}' is missing the 'path' argument")]
    MissingHttpPath { responder: String },

    #[error(
        "#[handles_middleware_attribute] is only supported on structs, but '{target}' is not a struct"
    )]
    HttpMiddlewareNotOnStruct { target: String },

    #[error("middleware '{middleware}' is missing the 'attribute' argument")]
    MissingMiddlewareHandles { middleware: String },

    #[error(
        "responder '{responder}' has a #[middleware(...)] attribute that must name exactly one middleware tag"
    )]
    MalformedMiddleware { responder: String },

    #[error(
        "responder '{responder}' attaches the middleware tag '{tag}', but no #[handles_middleware_attribute] handles it"
    )]
    UnknownMiddleware { responder: String, tag: String },

    #[error("responder '{responder}' has a malformed route path '{path}': {source}")]
    InvalidRoutePath {
        responder: String,
        path: String,
        #[source]
        source: InsertError,
    },

    #[error(
        "responder '{responder}' registers route path '{path}' on server '{server}', which conflicts with the already registered route path '{conflicting_path}'"
    )]
    ConflictingRoutePaths {
        conflicting_path: String,
        path: String,
        responder: String,
        server: String,
    },

    #[error(
        "responder '{responder}' registers '{method} {path}' on server '{server}', which is already registered by responder '{existing_responder}'"
    )]
    DuplicateRoute {
        existing_responder: String,
        method: String,
        path: String,
        responder: String,
        server: String,
    },

    #[error(
        "responders '{first}' and '{second}' both declare the route name '{name}'; each route name may identify at most one responder"
    )]
    DuplicateRouteName {
        name: String,
        first: String,
        second: String,
    },

    #[error(
        "responder '{responder}' declares the route name '{name}', which must be a snake_case identifier usable as a `routes` accessor"
    )]
    InvalidRouteName { name: String, responder: String },

    #[error(
        "parameter '{parameter}' of middleware '{middleware}' must be the current request, the next handler, or the routes"
    )]
    UnclassifiableMiddlewareParameter {
        middleware: String,
        parameter: String,
    },

    #[error("responder '{responder}' is missing the 'server' argument")]
    MissingHttpServer { responder: String },

    #[error(
        "responder '{responder}' names the server '{server}', which must be a snake_case identifier usable as a `routes` accessor"
    )]
    InvalidServerName { responder: String, server: String },

    #[error(
        "responder '{responder}' injects &Views, but the crate defines no #[renders_view]; a view must exist to be injected"
    )]
    ViewInjectedWithoutViews { responder: String },
}
