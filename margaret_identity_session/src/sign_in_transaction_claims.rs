use serde::Deserialize;
use serde_json::Value;

use margaret_oauth_vocabulary::code_verifier::CodeVerifier;
use margaret_registered_claims::registered_claims::RegisteredClaims;

#[derive(Deserialize)]
pub struct SignInTransactionClaims {
    pub code_verifier: CodeVerifier,
    pub nonce: String,
    pub state: String,
}

impl SignInTransactionClaims {
    #[must_use]
    pub fn to_payload(&self, registered: &RegisteredClaims) -> Value {
        let mut payload = registered.to_json();

        payload.insert(
            "code_verifier".to_string(),
            Value::String(self.code_verifier.secret().to_string()),
        );
        payload.insert("nonce".to_string(), Value::String(self.nonce.clone()));
        payload.insert("state".to_string(), Value::String(self.state.clone()));

        Value::Object(payload)
    }
}
