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
        "marker '{marker}' on responder '{responder}' must carry zero or one positional argument"
    )]
    MalformedMarker { responder: String, marker: String },

    #[error("responder '{responder}' has no #[responder] method")]
    MissingResponderMethod { responder: String },

    #[error(
        "parameter '{parameter}' of responder '{responder}' must be a route parameter, a session-authenticated parameter, or the current request"
    )]
    UnmarkedResponderParameter {
        responder: String,
        parameter: String,
    },

    #[error("route parameter '{parameter}' of responder '{responder}' is not a plain identifier")]
    RouteParameterNotIdentifier {
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
        "#[decides_crud_action] '{gate}' has no `type Subject = <struct>` associated type that resolves to a known model"
    )]
    CrudGateSubject { gate: String },

    #[error("model '{subject}' has more than one CRUD gate: '{first}' and '{second}'")]
    AmbiguousCrudGate {
        subject: String,
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
        "route parameter '{parameter}' of responder '{responder}' declares an intent but type '{written}' has no #[decides_crud_action] gate"
    )]
    MissingCrudGate {
        responder: String,
        parameter: String,
        written: String,
    },

    #[error(
        "route parameter '{parameter}' of responder '{responder}' is a raw String but declares an intent; intent only applies to model parameters"
    )]
    IntentOnRawParameter {
        responder: String,
        parameter: String,
    },

    #[error(
        "more than one #[provides_authenticated_actor] store is defined: '{first}' and '{second}'; exactly one is allowed"
    )]
    AmbiguousAuthenticatedActorStore { first: String, second: String },

    #[error(
        "responder '{responder}' needs the authenticated actor but no #[provides_authenticated_actor] store is defined"
    )]
    NoAuthenticatedActorStore { responder: String },

    #[error("#[decides_site_action] gate '{gate}' is missing its site action argument")]
    MissingSiteActionArgument { gate: String },

    #[error("site action '{action}' has more than one gate: '{first}' and '{second}'")]
    AmbiguousSiteActionGate {
        action: String,
        first: String,
        second: String,
    },

    #[error("responder '{responder}' has a #[can] attribute without a site action argument")]
    CanWithoutAction { responder: String },

    #[error(
        "responder '{responder}' is guarded by site action '{action}' but no #[decides_site_action] gate decides it"
    )]
    MissingSiteActionGate { responder: String, action: String },

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
