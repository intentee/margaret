use margaret_oauth_vocabulary::scope::Scope;

#[test]
fn accepts_a_scope_token() {
    assert_eq!(
        "artifacts:write"
            .parse::<Scope>()
            .expect("the scope is a scope token")
            .as_str(),
        "artifacts:write"
    );
}
