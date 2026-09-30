use serde_json::Value;
use serde_json::json;

#[must_use]
pub fn localhost_discovery() -> Value {
    json!({ "issuer": "https://localhost", "jwks_uri": "https://localhost/jwks" })
}
