use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;
use margaret_oauth_vocabulary::scope::Scope;

#[test]
fn rejects_a_scope_outside_the_scope_token_grammar() {
    for scope in ["read write", "quoted\"", "back\\slash"] {
        assert!(matches!(
            scope.parse::<Scope>(),
            Err(OAuthVocabularyError::ScopeCharacter)
        ));
    }
}
