use std::collections::BTreeSet;

use serde_json::Value;

use margaret_oauth_vocabulary::space_joined::space_joined;
use margaret_registered_claims::registered_claims::RegisteredClaims;

pub struct IssuedAccessTokenClaims {
    pub client_id: &'static str,
    pub scopes: BTreeSet<String>,
    pub subject: String,
}

impl IssuedAccessTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "client_id".to_string(),
            Value::String(self.client_id.to_string()),
        );
        payload.insert(
            "scope".to_string(),
            Value::String(space_joined(self.scopes.iter().map(String::as_str))),
        );
        payload.insert("sub".to_string(), Value::String(self.subject.clone()));

        Value::Object(payload)
    }
}
