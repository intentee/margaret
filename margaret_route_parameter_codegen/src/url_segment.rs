pub enum UrlSegment {
    CatchAllParameter(String),
    Literal(String),
    Parameter(String),
}
