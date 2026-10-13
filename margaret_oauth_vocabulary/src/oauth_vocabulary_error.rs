use thiserror::Error;

#[derive(Debug, Error)]
pub enum OAuthVocabularyError {
    #[error("the client secret contains a character outside the visible ascii range")]
    ClientSecretInvisibleCharacter,

    #[error("the client secret is empty")]
    EmptyClientSecret,
}
