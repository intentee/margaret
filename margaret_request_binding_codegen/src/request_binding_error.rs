use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum RequestBindingError {
    #[error("failed to read a binding attribute: {source}")]
    Attribute {
        #[from]
        source: AttributeError,
    },

    #[error(
        "parameter '{parameter}' of {subject} must be a route parameter, a form request, the current request, the peer SPIFFE id, the forwarder, or the routes"
    )]
    UnmarkedParameter { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} is not an injectable dependency; expected Arc<T> or Vec<Arc<dyn Trait>>"
    )]
    UnsupportedParameterShape { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} requests an injectable dependency, but no #[singleton] provides it"
    )]
    MissingProvider { subject: String, parameter: String },

    #[error(
        "route parameter #{parameter} of {subject} is missing `from = \"...\"`; it must name the path parameter it binds"
    )]
    RouteParameterMissingFrom { subject: String, parameter: String },

    #[error(
        "route parameter '{parameter}' of {subject} does not appear in the route path '{path}'"
    )]
    RouteParameterNotInPath {
        subject: String,
        parameter: String,
        path: String,
    },

    #[error(
        "route parameter '{parameter}' of {subject} has type '{written}', which has no #[provides_route_parameter]"
    )]
    MissingRouteParameterBinder {
        subject: String,
        parameter: String,
        written: String,
    },

    #[error(
        "form request argument #{parameter} of {subject} is missing `from = <source>`; it must name the request input source it validates"
    )]
    FormRequestMissingSource { subject: String, parameter: String },

    #[error(
        "form request argument #{parameter} of {subject} names an unknown request input source '{written}'; expected Form, Query, Json, or Cookie"
    )]
    UnknownRequestInput {
        subject: String,
        parameter: String,
        written: String,
    },

    #[error(
        "form request argument #{parameter} of {subject} reads the request body via '{input_source}', but a WebSocket upgrade handshake has no body; only `from = Query` or `from = Cookie` is available"
    )]
    FormRequestBodyUnavailable {
        subject: String,
        parameter: String,
        input_source: String,
    },

    #[error(
        "argument #{parameter} of {subject} has both #[route_parameter] and #[form_request]; an argument may use at most one"
    )]
    ConflictingArgumentMarkers { subject: String, parameter: String },

    #[error(
        "argument #{parameter} of {subject} is the peer SPIFFE id and must not also carry #[route_parameter] or #[form_request]"
    )]
    MarkedPeerSpiffeIdParameter { subject: String, parameter: String },

    #[error(
        "{subject} declares more than one peer SPIFFE id parameter; a request has exactly one peer identity"
    )]
    MultiplePeerSpiffeIdParameters { subject: String },

    #[error(
        "argument #{parameter} of {subject} requests the forwarder, but a WebSocket session builder cannot forward a request"
    )]
    ForwarderUnavailable { subject: String, parameter: String },

    #[error(
        "#[provides_route_parameter] '{binder}' has no `type Model = <struct>` associated type that resolves to a known model"
    )]
    RouteParameterBinderModel { binder: String },

    #[error(
        "#[provides_route_parameter] is only supported on structs, but '{binder}' is not a struct"
    )]
    RouteParameterBinderNotAStruct { binder: String },

    #[error("model '{model}' has more than one route parameter binder: '{first}' and '{second}'")]
    AmbiguousRouteParameterBinder {
        model: String,
        first: String,
        second: String,
    },
}
