use serde_json::Value;
use uuid::Uuid;

use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct ClientAssertionClaims {
    pub audience: &'static str,
    pub client_id: &'static str,
    pub expires_at: NumericDate,
    pub issued_at: NumericDate,
    pub jti: Uuid,
}

impl ClientAssertionClaims {
    #[must_use]
    pub fn to_payload(&self) -> Value {
        let mut payload = RegisteredClaims {
            aud: AudienceClaim::Single(self.audience.to_string()),
            exp: self.expires_at,
            iat: Some(self.issued_at),
            iss: self.client_id.to_string(),
            jti: Some(self.jti.to_string()),
            nbf: None,
        }
        .to_json();

        payload.insert("sub".to_string(), Value::String(self.client_id.to_string()));

        Value::Object(payload)
    }
}
