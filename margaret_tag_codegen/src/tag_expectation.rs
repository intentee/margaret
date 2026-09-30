use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::tag_kind::TagKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TagExpectation {
    Middleware,
    OAuthClient,
    TrustedIssuer,
}

impl TagExpectation {
    pub(crate) fn admits(self, kind: TagKind) -> bool {
        match self {
            Self::Middleware => matches!(kind, TagKind::Middleware),
            Self::OAuthClient => matches!(kind, TagKind::OAuthClient),
            Self::TrustedIssuer => matches!(kind, TagKind::TrustedIssuer(_)),
        }
    }
}

impl Display for TagExpectation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        let label = match self {
            Self::Middleware => "middleware handler",
            Self::OAuthClient => "oauth client",
            Self::TrustedIssuer => "trusted issuer",
        };

        formatter.write_str(label)
    }
}
