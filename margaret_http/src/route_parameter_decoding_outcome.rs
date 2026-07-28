pub(crate) enum RouteParameterDecodingOutcome {
    Decoded(String),
    NotUtf8,
}
