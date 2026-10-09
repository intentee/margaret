#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UrlSegment {
    CatchAllParameter {
        name: String,
        prefix: String,
    },
    Literal(String),
    Parameter {
        name: String,
        prefix: String,
        suffix: String,
    },
}
