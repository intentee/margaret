use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;

#[test]
fn rejects_a_client_secret_with_an_invisible_character() {
    assert!(matches!(
        "secret\n".parse::<ClientSecret>(),
        Err(OAuthVocabularyError::ClientSecretInvisibleCharacter)
    ));
}
