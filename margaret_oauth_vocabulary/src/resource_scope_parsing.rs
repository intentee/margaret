use crate::resource_scope::ResourceScope;
use crate::resource_scope_rejection::ResourceScopeRejection;

#[derive(Debug, Eq, PartialEq)]
pub enum ResourceScopeParsing {
    Accepted(ResourceScope),
    Rejected(ResourceScopeRejection),
}
