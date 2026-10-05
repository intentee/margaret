use std::str::FromStr;

use crate::oauth_vocabulary_error::OAuthVocabularyError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodeChallengeMethod {
    S256,
}

impl CodeChallengeMethod {
    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::S256 => "S256",
        }
    }
}

impl FromStr for CodeChallengeMethod {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value == Self::S256.wire_name() {
            Ok(Self::S256)
        } else {
            Err(OAuthVocabularyError::UnsupportedCodeChallengeMethod {
                value: value.to_string(),
            })
        }
    }
}
