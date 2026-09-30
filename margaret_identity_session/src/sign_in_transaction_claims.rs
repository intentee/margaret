use serde::Deserialize;
use serde_json::Value;
use url::Url;

use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Deserialize)]
pub struct SignInTransactionClaims {
    pub callback: Url,
    pub nonce: String,
    pub pkce_verifier: String,
    pub state: String,
}

impl SignInTransactionClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "callback".to_string(),
            Value::String(self.callback.to_string()),
        );
        payload.insert("nonce".to_string(), Value::String(self.nonce.clone()));
        payload.insert(
            "pkce_verifier".to_string(),
            Value::String(self.pkce_verifier.clone()),
        );
        payload.insert("state".to_string(), Value::String(self.state.clone()));

        Value::Object(payload)
    }
}
