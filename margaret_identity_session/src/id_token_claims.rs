use serde_json::Value;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct IdTokenClaims {
    pub auth_time: NumericDate,
    pub client_id: ClientId,
    pub nonce: Option<String>,
    pub subject: Uuid,
}

impl IdTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "auth_time".to_string(),
            Value::Number(self.auth_time.seconds_since_epoch().into()),
        );
        payload.insert(
            "azp".to_string(),
            Value::String(self.client_id.as_str().to_string()),
        );

        if let Some(nonce) = &self.nonce {
            payload.insert("nonce".to_string(), Value::String(nonce.clone()));
        }

        payload.insert("sub".to_string(), Value::String(self.subject.to_string()));

        Value::Object(payload)
    }
}
