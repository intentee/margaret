use serde_json::Value;
use serde_json::json;

use crate::pkce_challenge::pkce_challenge;
use crate::portal_callback::PORTAL_CALLBACK;

#[must_use]
pub fn portal_parameters() -> Value {
    json!({
        "client_id": "portal",
        "code_challenge": pkce_challenge(),
        "code_challenge_method": "S256",
        "nonce": "n-0S6_WzA2Mj",
        "redirect_uri": PORTAL_CALLBACK,
        "response_type": "code",
        "scope": "openid profile",
        "state": "af0ifjsldkj",
    })
}
