use matchit::InsertError;
use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;
use margaret_injection_codegen::injection_error::InjectionError;

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

    #[error(
        "parameter '{parameter}' of responder '{responder}' must be a route parameter, a form request, the current request, or the routes"
    )]
    UnmarkedResponderParameter {
        responder: String,
        parameter: String,
    },

    #[error(
        "argument #{parameter} of responder '{responder}' has both #[route_parameter] and #[form_request]; a responder argument may use at most one"
    )]
    ConflictingArgumentMarkers {
        responder: String,
        parameter: String,
    },

    #[error(
        "form request argument #{parameter} of responder '{responder}' is missing `from = <source>`; it must name the request input source it validates"
    )]
    FormRequestMissingSource {
        responder: String,
        parameter: String,
    },

    #[error(
        "form request argument #{parameter} of responder '{responder}' names an unknown request input source '{written}'; expected Form, Query, or Json"
    )]
    UnknownRequestInput {
        responder: String,
        parameter: String,
        written: String,
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
        "responder '{responder}' declares the route name '{name}', which must be a snake_case identifier usable as a `routes` accessor"
    )]
    InvalidRouteName { name: String, responder: String },

    #[error(
        "#[interceptor] '{interceptor}' has no `Box<dyn <marker trait>>` parameter on its #[process] method that resolves to a known marker trait"
    )]
    MissingInterceptedParameter { interceptor: String },

    #[error(
        "#[interceptor] '{interceptor}' has more than one intercepted `Box<dyn <marker trait>>` parameter on its #[process] method"
    )]
    MultipleInterceptedParameters { interceptor: String },

    #[error(
        "parameter '{parameter}' of #[interceptor] '{interceptor}' must be the intercepted value, the current request, or the routes"
    )]
    UnclassifiableInterceptorParameter {
        interceptor: String,
        parameter: String,
    },

    #[error("marker trait '{intercepted}' has more than one interceptor: '{first}' and '{second}'")]
    AmbiguousInterceptor {
        intercepted: String,
        first: String,
        second: String,
    },

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
}
