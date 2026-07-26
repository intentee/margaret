use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteUrlError {
    #[error(
        "route parameter '{parameter}' has the reserved path segment value '{value}'; `.` and `..` cannot form a URL path segment"
    )]
    ReservedPathSegment { parameter: String, value: String },
}
