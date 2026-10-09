use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;

#[test]
fn rejects_an_empty_client_secret() {
    assert!(matches!(
        "".parse::<ClientSecret>(),
        Err(OAuthVocabularyError::EmptyClientSecret)
    ));
}
