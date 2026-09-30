use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;
use margaret_oauth_vocabulary::scope::Scope;

#[test]
fn rejects_an_empty_scope() {
    assert!(matches!(
        "".parse::<Scope>(),
        Err(OAuthVocabularyError::EmptyScope)
    ));
}
