use margaret_oauth_vocabulary::scope::Scope;
use margaret_oauth_vocabulary::scope_parsing::ScopeParsing;
use margaret_oauth_vocabulary::scope_rejection::ScopeRejection;

#[test]
fn rejects_an_empty_scope() {
    assert_eq!(
        Scope::parse(""),
        ScopeParsing::Rejected(ScopeRejection::Empty)
    );
}
