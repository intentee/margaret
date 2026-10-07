use std::str::FromStr;

use crate::oauth_vocabulary_error::OAuthVocabularyError;
use crate::scope::Scope;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceScope {
    scope: Scope,
}

impl ResourceScope {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.scope.as_str()
    }
}

impl FromStr for ResourceScope {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let scope = value.parse::<Scope>()?;

        if scope.is_openid() {
            Err(OAuthVocabularyError::OpenidResourceScope)
        } else {
            Ok(Self { scope })
        }
    }
}
