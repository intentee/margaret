use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use crate::openid_scope::OPENID_SCOPE;
use crate::scope_parsing::ScopeParsing;
use crate::scope_rejection::ScopeRejection;

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
    pub fn parse(value: &str) -> ScopeParsing {
        if value.is_empty() {
            ScopeParsing::Rejected(ScopeRejection::Empty)
        } else if value.bytes().all(is_scope_octet) {
            ScopeParsing::Accepted(Self {
                value: value.to_string(),
            })
        } else {
            ScopeParsing::Rejected(ScopeRejection::Character)
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

impl<'de> Deserialize<'de> for Scope {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> Result<Self, TDeserializer::Error> {
        match Self::parse(&String::deserialize(deserializer)?) {
            ScopeParsing::Accepted(scope) => Ok(scope),
            ScopeParsing::Rejected(rejection) => Err(TDeserializer::Error::custom(rejection)),
        }
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
