use crate::scope::Scope;
use crate::scope_rejection::ScopeRejection;

#[derive(Debug, Eq, PartialEq)]
pub enum ScopeParsing {
    Accepted(Scope),
    Rejected(ScopeRejection),
}
