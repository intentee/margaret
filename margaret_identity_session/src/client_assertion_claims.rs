use serde_json::Value;
use uuid::Uuid;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::audience_claim::AudienceClaim;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;
use margaret_registered_claims::numeric_date::NumericDate;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct ClientAssertionClaims {
    pub audience: IssuerIdentifier,
    pub client_id: ClientId,
    pub expires_at: NumericDate,
    pub issued_at: NumericDate,
    pub jti: Uuid,
}

impl ClientAssertionClaims {
    #[must_use]
    pub fn to_payload(&self) -> Value {
        let mut payload = RegisteredClaims {
            aud: AudienceClaim::Single(self.audience.as_str().to_string()),
            exp: self.expires_at,
            iat: self.issued_at,
            iss: self.client_id.as_str().to_string(),
            nbf: None,
        }
        .to_json();

        payload.insert("jti".to_string(), Value::String(self.jti.to_string()));
        payload.insert(
            "sub".to_string(),
            Value::String(self.client_id.as_str().to_string()),
        );

        Value::Object(payload)
    }
}
