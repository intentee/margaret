#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UrlSegment {
    CatchAllParameter { name: String, prefix: String },
    Literal { decoded: String, encoded: String },
    Parameter { name: String, prefix: String },
}
