use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::oauth_vocabulary_error::OAuthVocabularyError;
use margaret_registered_claims::registered_claims_error::RegisteredClaimsError;

#[test]
fn rejects_an_empty_client_id() {
    assert!(matches!(
        "".parse::<ClientId>(),
        Err(OAuthVocabularyError::ClientIdNotAnAudience {
            source: RegisteredClaimsError::AudienceEmpty
        })
    ));
}
