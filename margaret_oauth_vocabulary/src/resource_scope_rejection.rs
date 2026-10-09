use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::scope_rejection::ScopeRejection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceScopeRejection {
    Openid,
    Scope(ScopeRejection),
}

impl Display for ResourceScopeRejection {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::Openid => formatter.write_str(
                "the openid scope asks for an identity, so it cannot grant access to a resource",
            ),
            Self::Scope(rejection) => rejection.fmt(formatter),
        }
    }
}
