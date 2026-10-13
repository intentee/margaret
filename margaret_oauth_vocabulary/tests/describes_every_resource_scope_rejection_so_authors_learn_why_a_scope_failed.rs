use margaret_oauth_vocabulary::resource_scope_rejection::ResourceScopeRejection;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[test]
fn describes_every_resource_scope_rejection_so_authors_learn_why_a_scope_failed() {
    assert_eq!(
        [
            ResourceScopeRejection::Openid,
            ResourceScopeRejection::Scope(ScopeRejection::Empty),
        ]
        .map(|rejection| rejection.to_string()),
        [
            "the openid scope asks for an identity, so it cannot grant access to a resource",
            "the scope is empty",
        ]
    );
}
