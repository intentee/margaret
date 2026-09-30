use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;

#[test]
fn rejects_a_client_id_with_an_invisible_character() {
    assert!(matches!(
        "client\u{7f}".parse::<ClientId>(),
        Err(OAuthVocabularyError::ClientIdInvisibleCharacter)
    ));
}
