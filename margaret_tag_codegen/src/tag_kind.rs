use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagKind {
    JwksClient,
    Middleware,
}

impl Display for TagKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        let label = match self {
            TagKind::JwksClient => "jwks endpoint provider",
            TagKind::Middleware => "middleware handler",
        };

        formatter.write_str(label)
    }
}
