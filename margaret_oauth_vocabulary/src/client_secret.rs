use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::FromStr;

use zeroize::Zeroizing;

use crate::oauth_vocabulary_error::OAuthVocabularyError;
use crate::visible_characters::visible_characters;

#[derive(Clone)]
pub struct ClientSecret {
    value: Zeroizing<String>,
}

impl ClientSecret {
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.value
    }
}

impl Debug for ClientSecret {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter
            .debug_struct("ClientSecret")
            .finish_non_exhaustive()
    }
}

impl FromStr for ClientSecret {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        if value.is_empty() {
            Err(OAuthVocabularyError::EmptyClientSecret)
        } else if visible_characters(value) {
            Ok(Self {
                value: Zeroizing::new(value.to_string()),
            })
        } else {
            Err(OAuthVocabularyError::ClientSecretInvisibleCharacter)
        }
    }
}
