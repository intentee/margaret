use serde_json::Value;
use serde_json::json;

use crate::pkce_verifier::PKCE_VERIFIER;
use crate::portal_callback::PORTAL_CALLBACK;

#[must_use]
pub fn code_exchange(code: &str) -> Value {
    json!({
        "code": code,
        "code_verifier": PKCE_VERIFIER,
        "grant_type": "authorization_code",
        "redirect_uri": PORTAL_CALLBACK,
    })
}
