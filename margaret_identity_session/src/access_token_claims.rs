use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::is_expired::IsExpired;

#[derive(Clone, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl AccessTokenClaims {
    #[must_use]
    pub fn to_json(&self) -> Value {
        let mut claims = Map::new();

        claims.insert("sub".to_string(), Value::String(self.sub.to_string()));
        claims.insert("exp".to_string(), Value::Number(self.exp.into()));
        claims.insert("iat".to_string(), Value::Number(self.iat.into()));

        Value::Object(claims)
    }
}

impl IsExpired for AccessTokenClaims {
    fn is_expired(&self, now: DateTime<Utc>) -> anyhow::Result<bool> {
        Ok(self.exp < now.timestamp())
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use chrono::Utc;
    use uuid::Uuid;

    use super::AccessTokenClaims;
    use crate::is_expired::IsExpired;

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(secs, 0).expect("a valid timestamp")
    }

    fn claims(exp: i64) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: Uuid::from_u128(1),
            exp,
            iat: 0,
        }
    }

    #[test]
    fn reports_expiry_relative_to_now() {
        assert_eq!(claims(101).is_expired(at(100)).ok(), Some(false));
        assert_eq!(claims(99).is_expired(at(100)).ok(), Some(true));
        assert_eq!(claims(100).is_expired(at(100)).ok(), Some(false));
    }
}
