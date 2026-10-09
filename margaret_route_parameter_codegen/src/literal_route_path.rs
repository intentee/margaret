#[derive(Debug, Eq, PartialEq)]
pub enum LiteralRoutePath {
    Literal(String),
    Parameterized,
}
