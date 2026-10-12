use margaret_oauth_vocabulary::resource_scope::ResourceScope;
use margaret_oauth_vocabulary::resource_scope_parsing::ResourceScopeParsing;
use margaret_oauth_vocabulary::resource_scope_rejection::ResourceScopeRejection;

#[test]
fn rejects_openid_as_a_resource_scope_since_it_asks_for_identity_not_access() {
    assert_eq!(
        ResourceScope::parse("openid"),
        ResourceScopeParsing::Rejected(ResourceScopeRejection::Openid)
    );
}
