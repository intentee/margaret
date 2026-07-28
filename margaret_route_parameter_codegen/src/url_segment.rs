pub enum UrlSegment {
    CatchAllParameter(String),
    Literal(String),
    SegmentParameter(String),
}
