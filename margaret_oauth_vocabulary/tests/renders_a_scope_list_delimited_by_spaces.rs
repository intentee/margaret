use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::scope_list_parsing::ScopeListParsing;

#[test]
fn renders_a_scope_list_delimited_by_spaces() {
    assert!(matches!(
        ScopeList::parse("profile openid"),
        ScopeListParsing::Accepted(scopes) if scopes.to_string() == "openid profile"
    ));
}
