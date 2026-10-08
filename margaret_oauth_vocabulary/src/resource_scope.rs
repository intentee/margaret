use crate::resource_scope_parsing::ResourceScopeParsing;
use crate::resource_scope_rejection::ResourceScopeRejection;
use crate::scope::Scope;
use crate::scope_parsing::ScopeParsing;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceScope {
    scope: Scope,
}

impl ResourceScope {
    #[must_use]
    pub fn parse(value: &str) -> ResourceScopeParsing {
        match Scope::parse(value) {
            ScopeParsing::Accepted(scope) if scope.is_openid() => {
                ResourceScopeParsing::Rejected(ResourceScopeRejection::Openid)
            }
            ScopeParsing::Accepted(scope) => ResourceScopeParsing::Accepted(Self { scope }),
            ScopeParsing::Rejected(rejection) => {
                ResourceScopeParsing::Rejected(ResourceScopeRejection::Scope(rejection))
            }
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.scope.as_str()
    }
}
