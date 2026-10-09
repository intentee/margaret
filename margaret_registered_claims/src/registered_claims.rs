use serde::Deserialize;
use serde::Deserializer;
use serde_json::Map;
use serde_json::Value;

use crate::audience_claim::AudienceClaim;
use crate::numeric_date::NumericDate;

fn present<'wire, TValue: Deserialize<'wire>, Source: Deserializer<'wire>>(
    deserializer: Source,
) -> Result<Option<TValue>, Source::Error> {
    TValue::deserialize(deserializer).map(Some)
}

fn numeric_date_member(date: NumericDate) -> Value {
    Value::Number(date.seconds_since_epoch().into())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegisteredClaims {
    pub aud: AudienceClaim,
    pub exp: NumericDate,
    #[serde(default, deserialize_with = "present")]
    pub iat: Option<NumericDate>,
    pub iss: String,
    #[serde(default, deserialize_with = "present")]
    pub jti: Option<String>,
    #[serde(default, deserialize_with = "present")]
    pub nbf: Option<NumericDate>,
}

impl RegisteredClaims {
    #[must_use]
    pub fn to_json(&self) -> Map<String, Value> {
        let mut members = Map::new();

        members.insert("aud".to_string(), self.aud.to_json());
        members.insert("exp".to_string(), numeric_date_member(self.exp));
        if let Some(iat) = self.iat {
            members.insert("iat".to_string(), numeric_date_member(iat));
        }

        members.insert("iss".to_string(), Value::String(self.iss.clone()));

        if let Some(jti) = &self.jti {
            members.insert("jti".to_string(), Value::String(jti.clone()));
        }

        if let Some(nbf) = self.nbf {
            members.insert("nbf".to_string(), numeric_date_member(nbf));
        }

        members
    }
}
