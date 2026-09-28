use chrono::DateTime;
use chrono::Utc;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use margaret_registered_claims::registered_claims::RegisteredClaims;
use margaret_token_issuance::token_issuance::TokenIssuance;

use crate::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;

pub struct AccessTokenStamp {
    pub jti: Uuid,
    pub registered: RegisteredClaims,
}

impl AccessTokenStamp {
    #[must_use]
    pub fn issued_by(issuance: &TokenIssuance, now: DateTime<Utc>) -> Self {
        Self {
            jti: Uuid::new_v4(),
            registered: issuance.registered_claims(now, ACCESS_TOKEN_LIFETIME_SECS),
        }
    }

    #[must_use]
    pub fn to_json(&self) -> Map<String, Value> {
        let mut members = self.registered.to_json();

        members.insert(
            "client_id".to_string(),
            Value::String(self.registered.iss.clone()),
        );
        members.insert("jti".to_string(), Value::String(self.jti.to_string()));

        members
    }
}
