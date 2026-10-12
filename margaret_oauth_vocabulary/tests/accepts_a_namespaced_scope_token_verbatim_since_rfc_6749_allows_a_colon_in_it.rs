use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;

#[test]
fn accepts_a_namespaced_scope_token_verbatim_since_rfc_6749_allows_a_colon_in_it() {
    assert!(matches!(
        Scope::parse("artifacts:write"),
        ScopeParsing::Accepted(scope) if scope.as_str() == "artifacts:write"
    ));
}
