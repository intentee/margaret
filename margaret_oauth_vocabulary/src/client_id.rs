use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::Error;

use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::audience_parsing::AudienceParsing;

use crate::client_id_parsing::ClientIdParsing;
use crate::client_id_rejection::ClientIdRejection;
use crate::visible_characters::visible_characters;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ClientId {
    audience: Audience,
}

impl ClientId {
    #[must_use]
    pub fn parse(value: &str) -> ClientIdParsing {
        let AudienceParsing::Accepted(audience) = Audience::parse(value) else {
            return ClientIdParsing::Rejected(ClientIdRejection::Empty);
        };

        if visible_characters(value) {
            ClientIdParsing::Accepted(Self { audience })
        } else {
            ClientIdParsing::Rejected(ClientIdRejection::InvisibleCharacter)
        }
    }

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

impl<'de> Deserialize<'de> for ClientId {
    fn deserialize<TDeserializer: Deserializer<'de>>(
        deserializer: TDeserializer,
    ) -> std::result::Result<Self, TDeserializer::Error> {
        match Self::parse(&String::deserialize(deserializer)?) {
            ClientIdParsing::Accepted(client_id) => Ok(client_id),
            ClientIdParsing::Rejected(rejection) => Err(TDeserializer::Error::custom(rejection)),
        }
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
