use std::str::FromStr;

use crate::oauth_vocabulary_error::OAuthVocabularyError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubjectTokenType {
    AccessToken,
    IdToken,
    Jwt,
}

impl SubjectTokenType {
    pub const ALL: [Self; 3] = [Self::AccessToken, Self::IdToken, Self::Jwt];

    #[must_use]
    pub fn urn(self) -> &'static str {
        match self {
            Self::AccessToken => "urn:ietf:params:oauth:token-type:access_token",
            Self::IdToken => "urn:ietf:params:oauth:token-type:id_token",
            Self::Jwt => "urn:ietf:params:oauth:token-type:jwt",
        }
    }
}

impl FromStr for SubjectTokenType {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|token_type| token_type.urn() == value)
            .ok_or_else(|| OAuthVocabularyError::UnsupportedTokenType {
                value: value.to_string(),
            })
    }
}
