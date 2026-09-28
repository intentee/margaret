use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::access_token_stamp::AccessTokenStamp;

#[derive(Clone, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
}

impl AccessTokenClaims {
    #[must_use]
    pub fn to_payload(&self, stamp: &AccessTokenStamp) -> Value {
        let mut payload = stamp.to_json();

        payload.insert("sub".to_string(), Value::String(self.sub.to_string()));

        Value::Object(payload)
    }
}
