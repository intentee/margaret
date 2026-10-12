use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;

#[test]
fn rejects_an_empty_client_secret_since_it_would_prove_nothing_about_the_client() {
    assert!(matches!(
        "".parse::<ClientSecret>(),
        Err(OAuthVocabularyError::EmptyClientSecret)
    ));
}
