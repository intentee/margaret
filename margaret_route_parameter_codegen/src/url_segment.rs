pub enum UrlSegment {
    CatchAll(String),
    Literal(String),
    Parameter(String),
}
