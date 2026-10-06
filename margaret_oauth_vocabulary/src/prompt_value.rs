use std::str::FromStr;

use crate::oauth_vocabulary_error::OAuthVocabularyError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PromptValue {
    Consent,
    Login,
    None,
}

impl PromptValue {
    pub const SUPPORTED: [Self; 3] = [Self::Consent, Self::Login, Self::None];

    #[must_use]
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Consent => "consent",
            Self::Login => "login",
            Self::None => "none",
        }
    }
}

impl FromStr for PromptValue {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::SUPPORTED
            .into_iter()
            .find(|supported| supported.wire_name() == value)
            .ok_or_else(|| OAuthVocabularyError::UnsupportedPromptValue {
                value: value.to_string(),
            })
    }
}
