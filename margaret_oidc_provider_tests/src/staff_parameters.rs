use serde_json::Value;
use serde_json::json;

use crate::constants::STAFF_CALLBACK;
use crate::pkce_challenge::pkce_challenge;

#[must_use]
pub fn staff_parameters(scope: &str) -> Value {
    json!({
        "client_id": "staff",
        "code_challenge": pkce_challenge(),
        "code_challenge_method": "S256",
        "redirect_uri": STAFF_CALLBACK,
        "response_type": "code",
        "scope": scope,
        "state": "af0ifjsldkj",
    })
}
