use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::tag_kind::TagKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagExpectation {
    Middleware,
    TokenIssuer,
}

impl TagExpectation {
    pub(crate) fn admits(self, kind: TagKind) -> bool {
        match self {
            Self::Middleware => matches!(kind, TagKind::Middleware),
            Self::TokenIssuer => matches!(kind, TagKind::JwksClient | TagKind::OidcIssuer),
        }
    }
}

impl Display for TagExpectation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let label = match self {
            Self::Middleware => "middleware handler",
            Self::TokenIssuer => "token issuer",
        };

        formatter.write_str(label)
    }
}
