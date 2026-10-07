use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::str::FromStr;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use margaret_registered_claims::audience::Audience;

use crate::oauth_vocabulary_error::OAuthVocabularyError;
use crate::visible_characters::visible_characters;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClientId {
    audience: Audience,
}

impl ClientId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.audience.as_str()
    }
}

impl Display for ClientId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        self.audience.fmt(formatter)
    }
}

impl FromStr for ClientId {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let audience = value
            .parse()
            .map_err(|source| OAuthVocabularyError::ClientIdNotAnAudience { source })?;

        if visible_characters(value) {
            Ok(Self { audience })
        } else {
            Err(OAuthVocabularyError::ClientIdInvisibleCharacter)
        }
    }
}

impl<'de> Deserialize<'de> for ClientId {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> std::result::Result<Self, TDeserializer::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(TDeserializer::Error::custom)
    }
}

impl Serialize for ClientId {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> std::result::Result<TSerializer::Ok, TSerializer::Error> {
        serializer.serialize_str(self.as_str())
    }
}
