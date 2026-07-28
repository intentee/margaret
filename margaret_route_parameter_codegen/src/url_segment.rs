pub enum UrlSegment {
    Literal(String),
    Parameter(String),
    WildcardParameter(String),
}
