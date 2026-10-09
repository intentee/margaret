use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;

#[test]
fn accepts_a_scope_token() {
    assert!(matches!(
        Scope::parse("artifacts:write"),
        ScopeParsing::Accepted(scope) if scope.as_str() == "artifacts:write"
    ));
}
