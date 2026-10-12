use matchit::InsertError;
use thiserror::Error;

use margaret_url_path::path_segment_rejection::PathSegmentRejection;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum RoutePathError {
    #[error("the route path conflicts with the already admitted route path '{conflicting_path}'")]
    ConflictingRoutePath { conflicting_path: String },
    #[error("the route path declares the parameter '{name}' more than once")]
    DuplicateParameterName { name: String },
    #[error("the route path parameter '{name}' is not a snake_case name")]
    InvalidParameterName { name: String },
    #[error("the route path does not start with '/'")]
    MissingLeadingSlash,
    #[error("the router cannot route the path: {source}")]
    RejectedByRouter {
        #[source]
        source: InsertError,
    },
    #[error("a responder already serves the route path")]
    ServedByResponder,
    #[error("a websocket session already serves the route path")]
    ServedByWebSocketSession,
    #[error("the route path segment '{segment}' can never be requested: {rejection}")]
    UnroutableSegment {
        segment: String,
        #[source]
        rejection: PathSegmentRejection,
    },
}
