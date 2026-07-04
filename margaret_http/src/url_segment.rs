pub enum UrlSegment {
    Literal(&'static str),
    Parameter(&'static str),
}
