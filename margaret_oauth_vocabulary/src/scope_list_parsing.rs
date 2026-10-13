use crate::scope_list::ScopeList;
use crate::scope_rejection::ScopeRejection;

#[derive(Debug, Eq, PartialEq)]
pub enum ScopeListParsing {
    Accepted(ScopeList),
    Rejected(ScopeRejection),
}
