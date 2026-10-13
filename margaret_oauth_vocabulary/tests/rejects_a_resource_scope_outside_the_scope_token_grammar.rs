use margaret_oauth_vocabulary::resource_scope::ResourceScope;
use margaret_oauth_vocabulary::resource_scope_parsing::ResourceScopeParsing;
use margaret_oauth_vocabulary::resource_scope_rejection::ResourceScopeRejection;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[test]
fn rejects_a_resource_scope_outside_the_scope_token_grammar() {
    assert_eq!(
        ResourceScope::parse("artifacts read"),
        ResourceScopeParsing::Rejected(ResourceScopeRejection::Scope(ScopeRejection::Character))
    );
}
