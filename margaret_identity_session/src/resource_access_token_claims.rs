use serde::Deserialize;
use serde_json::Value;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::scope_list::ScopeList;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ResourceAccessTokenClaims {
    pub client_id: ClientId,
    pub scope: ScopeList,
    #[serde(rename = "sub")]
    pub subject: String,
}

impl ResourceAccessTokenClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "client_id".to_string(),
            Value::String(self.client_id.as_str().to_string()),
        );
        payload.insert("scope".to_string(), Value::String(self.scope.to_string()));
        payload.insert("sub".to_string(), Value::String(self.subject.clone()));

        Value::Object(payload)
    }
}
