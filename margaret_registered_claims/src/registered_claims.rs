use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde_json::Map;
use serde_json::Value;

use crate::numeric_date::NumericDate;

fn present<'wire, Source: Deserializer<'wire>>(
    deserializer: Source,
) -> Result<Option<NumericDate>, Source::Error> {
    NumericDate::deserialize(deserializer).map(Some)
}

fn numeric_date_member(date: NumericDate) -> Value {
    Value::Number(date.seconds_since_epoch().into())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegisteredClaims {
    pub exp: NumericDate,
    pub iat: NumericDate,
    #[serde(default, deserialize_with = "present")]
    pub nbf: Option<NumericDate>,
}

impl RegisteredClaims {
    #[must_use]
    pub fn issued_at(now: DateTime<Utc>, lifetime_seconds: u32) -> Self {
        let iat = NumericDate::from(now);

        Self {
            exp: NumericDate::new(iat.seconds_since_epoch() + i64::from(lifetime_seconds)),
            iat,
            nbf: None,
        }
    }

    #[must_use]
    pub fn to_json(&self) -> Map<String, Value> {
        let mut members = Map::new();

        members.insert("exp".to_string(), numeric_date_member(self.exp));
        members.insert("iat".to_string(), numeric_date_member(self.iat));

        if let Some(nbf) = self.nbf {
            members.insert("nbf".to_string(), numeric_date_member(nbf));
        }

        members
    }
}
