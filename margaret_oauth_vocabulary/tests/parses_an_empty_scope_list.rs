use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn parses_an_empty_scope_list() {
    assert_eq!(
        "".parse::<ScopeList>()
            .expect("an empty scope list is valid"),
        ScopeList::default()
    );
}
