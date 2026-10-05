use serde_json::Value;
use serde_json::json;

use crate::pkce_challenge::pkce_challenge;
use crate::spa_callback::SPA_CALLBACK;

#[must_use]
pub fn spa_parameters() -> Value {
    json!({
        "client_id": "spa",
        "code_challenge": pkce_challenge(),
        "code_challenge_method": "S256",
        "redirect_uri": SPA_CALLBACK,
        "response_type": "code",
        "scope": "openid",
        "state": "af0ifjsldkj",
    })
}
