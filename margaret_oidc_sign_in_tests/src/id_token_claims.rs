use chrono::Utc;
use serde_json::Value;
use serde_json::json;

#[must_use]
pub fn id_token_claims(nonce: &str) -> Value {
    let now = Utc::now().timestamp();

    json!({
        "aud": "client:id",
        "azp": "client:id",
        "email": "user@example.test",
        "exp": now + 300,
        "iat": now,
        "iss": "https://localhost",
        "nonce": nonce,
        "sub": "subject",
    })
}
