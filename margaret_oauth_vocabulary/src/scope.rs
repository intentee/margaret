use std::str::FromStr;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use crate::oauth_vocabulary_error::OAuthVocabularyError;

const OPENID_SCOPE: &str = "openid";
const QUOTATION_MARK: u8 = 0x22;
const REVERSE_SOLIDUS: u8 = 0x5c;
const FIRST_SCOPE_OCTET: u8 = 0x21;
const LAST_SCOPE_OCTET: u8 = 0x7e;

fn is_scope_octet(octet: u8) -> bool {
    (FIRST_SCOPE_OCTET..=LAST_SCOPE_OCTET).contains(&octet)
        && octet != QUOTATION_MARK
        && octet != REVERSE_SOLIDUS
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scope {
    value: String,
}

impl Scope {
    #[must_use]
    pub fn openid() -> Self {
        Self {
            value: OPENID_SCOPE.to_string(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    #[must_use]
    pub fn is_openid(&self) -> bool {
        self.value == OPENID_SCOPE
    }
}

impl From<&Scope> for oauth2::Scope {
    fn from(scope: &Scope) -> Self {
        Self::new(scope.value.clone())
    }
}

impl FromStr for Scope {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(OAuthVocabularyError::EmptyScope)
        } else if value.bytes().all(is_scope_octet) {
            Ok(Self {
                value: value.to_string(),
            })
        } else {
            Err(OAuthVocabularyError::ScopeCharacter)
        }
    }
}

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(TDeserializer::Error::custom)
    }
}

impl Serialize for Scope {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        serializer.serialize_str(self.as_str())
    }
}
