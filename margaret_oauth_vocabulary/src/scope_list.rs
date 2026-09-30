use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use crate::oauth_vocabulary_error::OAuthVocabularyError;
use crate::scope::Scope;

const SCOPE_DELIMITER: char = ' ';

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScopeList {
    pub scopes: BTreeSet<Scope>,
}

impl Display for ScopeList {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            &self
                .scopes
                .iter()
                .map(Scope::as_str)
                .collect::<Vec<&str>>()
                .join(&SCOPE_DELIMITER.to_string()),
        )
    }
}

impl FromStr for ScopeList {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Ok(Self::default());
        }

        value
            .split(SCOPE_DELIMITER)
            .map(str::parse)
            .collect::<Result<BTreeSet<Scope>, OAuthVocabularyError>>()
            .map(|scopes| Self { scopes })
    }
}

impl<'de> Deserialize<'de> for ScopeList {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(TDeserializer::Error::custom)
    }
}

impl Serialize for ScopeList {
    fn serialize<TSerializer: Serializer>(
        &self,
        serializer: TSerializer,
    ) -> Result<TSerializer::Ok, TSerializer::Error> {
        serializer.collect_str(self)
    }
}
