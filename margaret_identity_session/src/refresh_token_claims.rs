use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Clone, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
}

impl RefreshTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert("sub".to_string(), Value::String(self.sub.to_string()));

        Value::Object(payload)
    }
}
