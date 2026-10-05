use margaret_oauth_vocabulary::resource_scope::ResourceScope;

#[test]
fn rejects_a_resource_scope_outside_the_scope_token_grammar() {
    assert_eq!(
        "artifacts read"
            .parse::<ResourceScope>()
            .expect_err("a resource scope is a scope token")
            .to_string(),
        "the scope contains a character outside the scope token grammar of RFC 6749"
    );
}
