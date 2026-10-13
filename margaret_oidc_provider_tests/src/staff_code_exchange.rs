use serde_json::Value;
use serde_json::json;

use crate::pkce_verifier::PKCE_VERIFIER;
use crate::staff_callback::STAFF_CALLBACK;

#[must_use]
pub fn staff_code_exchange(code: &str) -> Value {
    json!({
        "code": code,
        "code_verifier": PKCE_VERIFIER,
        "grant_type": "authorization_code",
        "redirect_uri": STAFF_CALLBACK,
    })
}
