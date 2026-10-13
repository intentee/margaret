use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SessionAccessTokenClaims {
    #[serde(with = "chrono::serde::ts_seconds")]
    pub auth_time: DateTime<Utc>,
    pub sid: Uuid,
    pub sub: Uuid,
}

impl SessionAccessTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "auth_time".to_string(),
            Value::from(self.auth_time.timestamp()),
        );
        payload.insert(
            "client_id".to_string(),
            Value::String(registered.iss.clone()),
        );
        payload.insert("sid".to_string(), Value::String(self.sid.to_string()));
        payload.insert("sub".to_string(), Value::String(self.sub.to_string()));

        Value::Object(payload)
    }
}
