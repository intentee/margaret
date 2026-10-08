use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use crate::scope::Scope;
use crate::scope_list_parsing::ScopeListParsing;
use crate::scope_parsing::ScopeParsing;
use crate::space_delimited::space_delimited;
use crate::space_joined::space_joined;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScopeList {
    pub scopes: BTreeSet<Scope>,
}

impl ScopeList {
    #[must_use]
    pub fn parse(value: &str) -> ScopeListParsing {
        let mut scopes = BTreeSet::new();

        for segment in space_delimited(value) {
            match Scope::parse(segment) {
                ScopeParsing::Accepted(scope) => {
                    scopes.insert(scope);
                }
                ScopeParsing::Rejected(rejection) => return ScopeListParsing::Rejected(rejection),
            }
        }

        ScopeListParsing::Accepted(Self { scopes })
    }
}

impl Display for ScopeList {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&space_joined(self.scopes.iter().map(Scope::as_str)))
    }
}

impl<'de> Deserialize<'de> for ScopeList {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        match Self::parse(&String::deserialize(deserializer)?) {
            ScopeListParsing::Accepted(scopes) => Ok(scopes),
            ScopeListParsing::Rejected(rejection) => Err(TDeserializer::Error::custom(rejection)),
        }
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
