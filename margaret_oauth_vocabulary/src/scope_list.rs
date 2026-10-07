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
use crate::space_delimited::space_delimited;
use crate::space_joined::space_joined;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScopeList {
    pub scopes: BTreeSet<Scope>,
}

impl Display for ScopeList {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&space_joined(self.scopes.iter().map(Scope::as_str)))
    }
}

impl FromStr for ScopeList {
    type Err = OAuthVocabularyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        space_delimited(value).map(|scopes| Self { scopes })
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
