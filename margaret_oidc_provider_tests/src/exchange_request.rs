use serde_json::Value;
use serde_json::json;

#[must_use]
pub fn exchange_request(subject_token: &str) -> Value {
    json!({
        "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
        "subject_token": subject_token,
        "subject_token_type": "urn:ietf:params:oauth:token-type:id_token",
    })
}
