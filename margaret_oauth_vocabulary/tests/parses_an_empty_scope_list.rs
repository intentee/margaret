use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_oauth_vocabulary::scope_list_parsing::ScopeListParsing;

#[test]
fn parses_an_empty_scope_list() {
    assert_eq!(
        ScopeList::parse(""),
        ScopeListParsing::Accepted(ScopeList::default())
    );
}
