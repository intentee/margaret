use matchit::InsertError;
use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum HttpCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("#[responds_to_http] is only supported on structs, but '{target}' is not a struct")]
    RespondsToHttpNotOnStruct { target: String },

    #[error("responder '{responder}' is missing the 'method' argument")]
    MissingHttpMethod { responder: String },

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

    #[error("responder '{responder}' has no #[responder] method")]
    MissingResponderMethod { responder: String },

    #[error(
        "parameter '{parameter}' of responder '{responder}' must be a route parameter or the current request"
    )]
    UnmarkedResponderParameter {
        responder: String,
        parameter: String,
    },

    #[error(
        "route parameter #{parameter} of responder '{responder}' is missing `from = \"...\"`; it must name the path parameter it binds"
    )]
    RouteParameterMissingFrom {
        responder: String,
        parameter: String,
    },

    #[error(
        "route parameter '{parameter}' of responder '{responder}' does not appear in the route path '{path}'"
    )]
    RouteParameterNotInPath {
        responder: String,
        parameter: String,
        path: String,
    },

    #[error("responder '{responder}' has a malformed route path '{path}': {source}")]
    InvalidRoutePath {
        responder: String,
        path: String,
        #[source]
        source: InsertError,
    },

    #[error(
        "#[provides_route_parameter] '{binder}' has no `type Model = <struct>` associated type that resolves to a known model"
    )]
    HttpRouteParameterBinderModel { binder: String },

    #[error("model '{model}' has more than one route parameter binder: '{first}' and '{second}'")]
    AmbiguousHttpRouteParameterBinder {
        model: String,
        first: String,
        second: String,
    },

    #[error(
        "route parameter '{parameter}' of responder '{responder}' has type '{written}', which has no #[provides_route_parameter]"
    )]
    MissingHttpRouteParameterBinder {
        responder: String,
        parameter: String,
        written: String,
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
        "#[intercepts] '{interceptor}' has no `type Intercepted = dyn <marker trait>` associated type that resolves to a known marker trait"
    )]
    MissingInterceptedType { interceptor: String },

    #[error("marker trait '{intercepted}' has more than one interceptor: '{first}' and '{second}'")]
    AmbiguousInterceptor {
        intercepted: String,
        first: String,
        second: String,
    },

    #[error("responder '{responder}' is missing the 'server' argument")]
    MissingHttpServer { responder: String },
}
