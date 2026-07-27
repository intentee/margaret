use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoutePathError {
    #[error("a route path must start with '/'")]
    MissingLeadingSlash,

    #[error("a route path must not contain an empty segment")]
    EmptySegment,

    #[error("a route path must not contain a dot segment")]
    DotSegment,

    #[error("a route path must not contain percent encoding")]
    PercentEncoding,

    #[error("a route path must not contain a backslash")]
    Backslash,

    #[error("a route path must not contain a query or fragment delimiter")]
    QueryOrFragment,

    #[error("a route path must not contain control characters")]
    ControlCharacter,
}
