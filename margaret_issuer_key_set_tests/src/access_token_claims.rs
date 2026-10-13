use serde_json::Value;
use serde_json::json;

use crate::access_token_audience::ACCESS_TOKEN_AUDIENCE;
use crate::access_token_issuer::ACCESS_TOKEN_ISSUER;
use crate::fixture_now::fixture_now;

#[must_use]
pub fn access_token_claims() -> Value {
    let now = fixture_now().seconds_since_epoch();

    json!({
        "aud": ACCESS_TOKEN_AUDIENCE,
        "exp": now + 600,
        "iat": now,
        "iss": ACCESS_TOKEN_ISSUER,
        "sub": "subject",
    })
}
