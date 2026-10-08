use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[test]
fn rejects_a_scope_outside_the_scope_token_grammar() {
    for scope in ["read write", "quoted\"", "back\\slash"] {
        assert_eq!(
            Scope::parse(scope),
            ScopeParsing::Rejected(ScopeRejection::Character)
        );
    }
}
