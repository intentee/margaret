use margaret_attributes::attribute_error::AttributeError;
use matchit::InsertError;
use thiserror::Error;

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

    #[error("#[http_middleware] is only supported on structs, but '{target}' is not a struct")]
    HttpMiddlewareNotOnStruct { target: String },

    #[error("middleware '{middleware}' is missing the 'handles' argument")]
    MissingMiddlewareHandles { middleware: String },

    #[error("middleware '{middleware}' is missing the 'priority' argument")]
    MissingMiddlewarePriority { middleware: String },

    #[error("middleware '{middleware}' has a 'priority' that is not an integer literal")]
    MalformedMiddlewarePriority { middleware: String },

    #[error(
        "marker '{marker}' on responder '{responder}' must carry zero or one positional argument"
    )]
    MalformedMarker { responder: String, marker: String },

    #[error("responder '{responder}' has no #[responder] method")]
    MissingResponderMethod { responder: String },

    #[error(
        "parameter '{parameter}' of responder '{responder}' is not marked #[route_parameter]; every responder parameter must be a route parameter"
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
}
