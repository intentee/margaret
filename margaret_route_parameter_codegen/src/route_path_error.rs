use margaret_url_path::path_segment_rejection::PathSegmentRejection;
use thiserror::Error;

#[derive(Debug, Eq, Error, PartialEq)]
pub enum RoutePathError {
    #[error("the catch-all parameter '{name}' is not at the end of the route path")]
    CatchAllNotLast { name: String },
    #[error("the route path parameter '{name}' is not a snake_case identifier")]
    InvalidParameterName { name: String },
    #[error("the route path does not start with '/'")]
    MissingLeadingSlash,
    #[error("the route path parameters '{first}' and '{second}' share one path segment")]
    ParametersShareSegment { first: String, second: String },
    #[error("the route path segment '{segment}' can never be requested: {rejection}")]
    UnroutableSegment {
        segment: String,
        #[source]
        rejection: PathSegmentRejection,
    },
}
