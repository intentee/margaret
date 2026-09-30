use margaret_oauth_vocabulary::scope_list::ScopeList;

#[test]
fn rejects_a_scope_list_with_an_empty_scope() {
    assert_eq!(
        serde_json::from_value::<ScopeList>(serde_json::json!("openid  profile"))
            .expect_err("two delimiters in a row enclose an empty scope")
            .to_string(),
        "the scope is empty"
    );
}
